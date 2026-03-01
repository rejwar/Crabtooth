// src/config.rs
// ─── App wide constants ────────────────────────────────────────────

use uuid::Uuid;

/// BLE Service UUID — দুটো device এ same থাকতে হবে
pub const SERVICE_UUID: Uuid = uuid::uuid!("0000fee0-0000-1000-8000-00805f9b34fb");

/// Message পাঠানোর Characteristic UUID
pub const MSG_WRITE_UUID: Uuid = uuid::uuid!("0000fee1-0000-1000-8000-00805f9b34fb");

/// Message receive করার Characteristic UUID  
pub const MSG_NOTIFY_UUID: Uuid = uuid::uuid!("0000fee2-0000-1000-8000-00805f9b34fb");

/// Scan করার সময় (সেকেন্ড)
pub const SCAN_DURATION_SECS: u64 = 6;

/// App এর নাম — BLE advertisement এ দেখা যাবে
pub const APP_NAME: &str = "bt-chat";

/// সর্বোচ্চ message length (BLE MTU limit)
pub const MAX_MSG_LEN: usize = 512;
