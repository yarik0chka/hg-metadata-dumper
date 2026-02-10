pub fn fmt_size(bytes: usize) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let b = bytes as f64;
    if b >= GB {
        format!("{} bytes ({:.2} GB)", bytes, b / GB)
    } else if b >= MB {
        format!("{} bytes ({:.2} MB)", bytes, b / MB)
    } else if b >= KB {
        format!("{} bytes ({:.2} KB)", bytes, b / KB)
    } else {
        format!("{} bytes", bytes)
    }
}

pub fn fmt_bytes_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}
