// src/chat.rs
// ─── Chat Loop ─────────────────────────────────────────────────────
//  connect হওয়ার পরে এই function টা চালু হয়
//  একই সাথে message পাঠানো ও receive করা handle করে

use anyhow::Result;
use std::io::{self, BufRead};
use tokio::sync::mpsc;

use crate::bluetooth::ChatSession;
use crate::ui;

/// Main chat loop
/// `session`   → connected BLE session
/// `my_name`   → user এর নাম
/// `peer_name` → অন্য device এর নাম
pub async fn run(session: ChatSession, my_name: &str) -> Result<()> {
    let peer_name = session.device_name().to_string();

    ui::show_chat_header(&peer_name);

    // Incoming message receiver
    let mut incoming = session.subscribe_messages().await?;

    // Keyboard input → একটা channel এ পাঠাবো (stdin blocking হয়, তাই আলাদা thread)
    let (input_tx, mut input_rx) = mpsc::channel::<String>(16);

    let _input_thread = std::thread::spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(text) => {
                    if input_tx.blocking_send(text).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    // ── Main loop ───────────────────────────────────────────────────
    ui::show_prompt(my_name);

    loop {
        tokio::select! {

            // অন্যজনের message এলো
            Some(msg) = incoming.recv() => {
                // prompt মুছে message দেখাও, তারপর prompt আবার দেখাও
                print!("\r");
                ui::print_peer_message(&peer_name, &msg);
                ui::show_prompt(my_name);
            }

            // User কিছু টাইপ করলো
            Some(input) = input_rx.recv() => {
                let text = input.trim().to_string();

                // quit command
                if text == "/quit" || text == "quit" {
                    ui::print_system("Chat বন্ধ করা হচ্ছে...");
                    session.disconnect().await?;
                    ui::show_disconnected();
                    break;
                }

                if text.is_empty() {
                    ui::show_prompt(my_name);
                    continue;
                }

                // Message পাঠাও
                match session.send(&text).await {
                    Ok(_) => {
                        ui::print_my_message(my_name, &text);
                    }
                    Err(e) => {
                        ui::print_error(&format!("পাঠানো যায়নি: {}", e));
                    }
                }

                ui::show_prompt(my_name);
            }
        }
    }

    Ok(())
}
