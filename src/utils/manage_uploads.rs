use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_id() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    return nanos.to_string();
}
