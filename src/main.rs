// src/main.rs
// ─── bt-chat Entry Point ───────────────────────────────────────────
//
//  চালানোর নিয়ম:
//    cargo run
//
//  তারপর:
//    1. নিজের নাম দাও
//    2. Scan হবে, device list দেখাবে
//    3. Number টাইপ করে device select করো
//    4. Chat শুরু!

mod bluetooth;
mod chat;
mod config;
mod ui;

use anyhow::Result;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<()> {
    ui::clear_screen();
    ui::show_banner();

    // নিজের নাম নাও
    let my_name = ask_name()?;

    // Scan করো
    ui::show_scanning(config::SCAN_DURATION_SECS);
    let devices = bluetooth::scan_for_devices().await?;
    ui::scanning_done(devices.len());

    if devices.is_empty() {
        ui::print_system("অন্য device এ bt-chat চালু করো, তারপর আবার চেষ্টা করো।");
        return Ok(());
    }

    // Device list দেখাও
    let names: Vec<String> = devices.iter().map(|d| d.name.clone()).collect();
    ui::show_device_list(&names);

    // Device select করো
    let choice = ask_device_choice(devices.len())?;
    let selected = devices.into_iter().nth(choice).unwrap();

    // Connect করো
    ui::show_connecting(&selected.name);
    let session =
        bluetooth::ChatSession::connect(selected.peripheral, selected.name.clone()).await?;
    ui::show_connected(&selected.name);

    // Chat শুরু করো
    chat::run(session, &my_name).await?;

    Ok(())
}

// ── Helpers ────────────────────────────────────────────────────────

fn ask_name() -> Result<String> {
    print!("  তোমার নাম লেখো: ");
    io::stdout().flush()?;

    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim().to_string();

    if name.is_empty() {
        return Ok("User".to_string());
    }

    println!();
    Ok(name)
}

fn ask_device_choice(count: usize) -> Result<usize> {
    loop {
        print!("  Device number লেখো (0–{}): ", count - 1);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().parse::<usize>() {
            Ok(n) if n < count => return Ok(n),
            _ => ui::print_error(&format!("0 থেকে {} এর মধ্যে লেখো", count - 1)),
        }
    }
}
