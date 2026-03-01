// src/bluetooth/session.rs
// ─── Active BLE Chat Session ───────────────────────────────────────
//  একটা connected device এর সাথে message পাঠানো ও নেওয়ার কাজ এখানে

use anyhow::{anyhow, Result};
use btleplug::api::{Peripheral as _, WriteType};
use btleplug::platform::Peripheral;
use futures::StreamExt;
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::config::{MAX_MSG_LEN, MSG_NOTIFY_UUID, MSG_WRITE_UUID, SERVICE_UUID};

/// Connected device এর chat session
pub struct ChatSession {
    device: Peripheral,
    device_name: String,
}

impl ChatSession {
    /// Device এ connect করো এবং session তৈরি করো
    pub async fn connect(device: Peripheral, name: String) -> Result<Self> {
        device.connect().await?;
        device.discover_services().await?;
        Ok(Self {
            device,
            device_name: name,
        })
    }

    pub fn device_name(&self) -> &str {
        &self.device_name
    }

    /// Disconnect করো
    pub async fn disconnect(&self) -> Result<()> {
        self.device.disconnect().await?;
        Ok(())
    }

    /// Message পাঠাও
    pub async fn send(&self, msg: &str) -> Result<()> {
        if msg.is_empty() {
            return Ok(());
        }

        let text = if msg.len() > MAX_MSG_LEN {
            &msg[..MAX_MSG_LEN]
        } else {
            msg
        };

        // Write Characteristic খোঁজো
        let chars = self.device.characteristics();
        let write_char = chars
            .iter()
            .find(|c| c.uuid == MSG_WRITE_UUID)
            .ok_or(anyhow!("Write characteristic পাওয়া যায়নি"))?
            .clone();

        self.device
            .write(&write_char, text.as_bytes(), WriteType::WithoutResponse)
            .await?;

        Ok(())
    }

    /// Incoming messages এর জন্য একটা channel তৈরি করো
    /// background task message পেলে channel এ পাঠাবে
    pub async fn subscribe_messages(&self) -> Result<Receiver<String>> {
        let chars = self.device.characteristics();
        let notify_char = chars
            .iter()
            .find(|c| c.uuid == MSG_NOTIFY_UUID)
            .ok_or(anyhow!("Notify characteristic পাওয়া যায়নি"))?
            .clone();

        self.device.subscribe(&notify_char).await?;

        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel(32);
        let device_clone = self.device.clone();

        // Background task — message আসলে channel এ দেয়
        tokio::spawn(async move {
            if let Ok(mut stream) = device_clone.notifications().await {
                while let Some(data) = stream.next().await {
                    if data.uuid == MSG_NOTIFY_UUID {
                        let msg = String::from_utf8_lossy(&data.value).to_string();
                        if tx.send(msg).await.is_err() {
                            break; // receiver বন্ধ হয়ে গেছে
                        }
                    }
                }
            }
        });

        Ok(rx)
    }
}
