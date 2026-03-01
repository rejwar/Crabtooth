// src/ui.rs
// ─── Terminal UI ───────────────────────────────────────────────────
//  সব print/display logic এক জায়গায়

use chrono::Local;
use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

// ── Startup ──────────────────────────────────────────────────────

pub fn clear_screen() {
    execute!(
        stdout(),
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .ok();
}

pub fn show_banner() {
    let banner = r#"
  ╔══════════════════════════════════════╗
  ║         🔵  bt-chat  v0.1            ║
  ║   Bluetooth CLI Chat  •  Rust        ║
  ╚══════════════════════════════════════╝"#;

    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print(banner),
        Print("\n\n"),
        ResetColor,
    )
    .ok();
}

// ── Scanning ─────────────────────────────────────────────────────

pub fn show_scanning(secs: u64) {
    execute!(
        stdout(),
        SetForegroundColor(Color::Yellow),
        Print(format!("🔍  Scanning {} সেকেন্ড...\n", secs)),
        ResetColor,
    )
    .ok();
}

pub fn scanning_done(count: usize) {
    if count == 0 {
        execute!(
            stdout(),
            SetForegroundColor(Color::Red),
            Print("❌  কোনো bt-chat device পাওয়া যায়নি।\n"),
            ResetColor,
        )
        .ok();
    } else {
        execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            Print(format!("✅  {} টা device পাওয়া গেছে!\n\n", count)),
            ResetColor,
        )
        .ok();
    }
}

// ── Device List ──────────────────────────────────────────────────

pub fn show_device_list(devices: &[String]) {
    println!("  ┌─── পাওয়া Devices ──────────────────┐");
    for (i, name) in devices.iter().enumerate() {
        execute!(
            stdout(),
            Print("  │  "),
            SetForegroundColor(Color::White),
            Print(format!("[{}] ", i)),
            SetForegroundColor(Color::Cyan),
            Print(format!("{}\n", name)),
            ResetColor,
        )
        .ok();
    }
    println!("  └────────────────────────────────────┘\n");
}

// ── Connection ───────────────────────────────────────────────────

pub fn show_connecting(name: &str) {
    execute!(
        stdout(),
        SetForegroundColor(Color::Yellow),
        Print(format!("\n  🔗 {} এ connecting...\n", name)),
        ResetColor,
    )
    .ok();
}

pub fn show_connected(name: &str) {
    execute!(
        stdout(),
        SetForegroundColor(Color::Green),
        Print(format!("  ✅ Connected: {}\n", name)),
        ResetColor,
    )
    .ok();
}

pub fn show_disconnected() {
    execute!(
        stdout(),
        SetForegroundColor(Color::DarkYellow),
        Print("\n  🔴 Disconnected. Goodbye!\n"),
        ResetColor,
    )
    .ok();
}

// ── Chat ─────────────────────────────────────────────────────────

pub fn show_chat_header(peer_name: &str) {
    let line = "─".repeat(44);
    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print(format!("\n  {}\n", line)),
        Print(format!("  💬  {} এর সাথে chat\n", peer_name)),
        Print(format!("  {}  /quit লিখলে বের হবে\n", " ".repeat(2))),
        Print(format!("  {}\n\n", line)),
        ResetColor,
    )
    .ok();
}

/// নিজের পাঠানো message
pub fn print_my_message(my_name: &str, msg: &str) {
    let time = Local::now().format("%H:%M").to_string();
    execute!(
        stdout(),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("  [{}]  ", time)),
        SetForegroundColor(Color::Green),
        Print(format!("{} ▶  ", my_name)),
        ResetColor,
        Print(format!("{}\n", msg)),
    )
    .ok();
}

/// অন্যজনের message
pub fn print_peer_message(peer_name: &str, msg: &str) {
    let time = Local::now().format("%H:%M").to_string();
    execute!(
        stdout(),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("  [{}]  ", time)),
        SetForegroundColor(Color::Blue),
        Print(format!("{} ◀  ", peer_name)),
        ResetColor,
        Print(format!("{}\n", msg)),
    )
    .ok();
}

/// System notification (connect/disconnect ইত্যাদি)
pub fn print_system(msg: &str) {
    execute!(
        stdout(),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("  ◈  {}\n", msg)),
        ResetColor,
    )
    .ok();
}

/// Error message
pub fn print_error(msg: &str) {
    execute!(
        stdout(),
        SetForegroundColor(Color::Red),
        Print(format!("\n  ❌  {}\n\n", msg)),
        ResetColor,
    )
    .ok();
}

/// Input prompt
pub fn show_prompt(my_name: &str) {
    execute!(
        stdout(),
        SetForegroundColor(Color::Green),
        Print(format!("\n  {} ▶  ", my_name)),
        ResetColor,
    )
    .ok();
    stdout().flush().ok();
}
