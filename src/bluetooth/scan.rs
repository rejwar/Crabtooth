// src/bluetooth/scan.rs
// ─── BLE Device Scanner ────────────────────────────────────────────

use anyhow::{anyhow, Result};
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Manager, Peripheral};
use std::time::Duration;
use tokio::time::sleep;

use crate::config::{APP_NAME, SCAN_DURATION_SECS};

/// একটি পাওয়া device এর তথ্য
#[derive(Debug, Clone)]
pub struct FoundDevice {
    pub name: String,
    pub peripheral: Peripheral,
}

/// আশেপাশে bt-chat app চালানো device খোঁজে
/// শুধু APP_NAME দিয়ে advertise করা device return করে
pub async fn scan_for_devices() -> Result<Vec<FoundDevice>> {
    // Bluetooth manager ও adapter নাও
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;

    let adapter = adapters
        .into_iter()
        .next()
        .ok_or(anyhow!("কোনো Bluetooth adapter পাওয়া যায়নি"))?;

    // Scan শুরু করো
    adapter.start_scan(ScanFilter::default()).await?;
    sleep(Duration::from_secs(SCAN_DURATION_SECS)).await;
    adapter.stop_scan().await?;

    // পাওয়া সব device এর নাম চেক করো
    let peripherals = adapter.peripherals().await?;

    let mut found = Vec::new();

    for p in peripherals {
        if let Ok(Some(props)) = p.properties().await {
            let name = props.local_name.unwrap_or_default();
            // শুধু bt-chat device নাও
            if name.starts_with(APP_NAME) {
                found.push(FoundDevice {
                    name,
                    peripheral: p,
                });
            }
        }
    }

    Ok(found)
}
