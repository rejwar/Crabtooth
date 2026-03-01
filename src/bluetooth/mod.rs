// src/bluetooth/mod.rs
// ─── Bluetooth layer ───────────────────────────────────────────────
//  scan()        → আশেপাশের bt-chat device খোঁজে
//  connect()     → নির্দিষ্ট device এ connect করে
//  send_msg()    → message পাঠায়
//  recv_msgs()   → message receive করার stream দেয়

pub mod scan;
pub mod session;

pub use scan::scan_for_devices;
pub use session::ChatSession;
