const DELTA: u32 = 0x9E3779B9;

fn fix_key(key: &[u8]) -> Vec<u8> {
    if key.len() == 16 {
        return key.to_vec();
    }
    if key.len() < 16 {
        let mut fixed = key.to_vec();
        fixed.resize(16, 0);
        return fixed;
    }
    key[..16].to_vec()
}

fn to_uint32_array(data: &[u8], include_length: bool) -> Vec<u32> {
    let length = data.len();
    let n = if (length & 3) == 0 {
        length >> 2
    } else {
        (length >> 2) + 1
    };

    let end_len = if include_length { n + 1 } else { n };
    let mut result = vec![0u32; end_len];

    if include_length {
        result[n] = length as u32;
    }

    for (i, &b) in data.iter().enumerate() {
        result[i >> 2] |= (b as u32) << ((i & 3) << 3);
    }
    result
}

fn to_byte_array(data: &[u32], include_length: bool) -> Option<Vec<u8>> {
    let mut n = data.len() << 2;

    if include_length {
        if data.is_empty() {
            return Some(Vec::new());
        }
        let last = data.last().copied().unwrap_or(0);
        let m = last as usize;
        n -= 4;
        if m < n.saturating_sub(3) || m > n {
            return None;
        }
        n = m;
    }

    let mut result = vec![0u8; n];
    for i in 0..n {
        result[i] = ((data[i >> 2] >> ((i & 3) << 3)) & 0xFF) as u8;
    }
    Some(result)
}

fn mx(sum_value: u32, y: u32, z: u32, p: usize, e: u32, k: &[u32]) -> u32 {
    let p_mask = (p as u32) & 3;
    let k_val = k[(p_mask ^ e) as usize];

    let part1 = (z >> 5) ^ (y << 2);
    let part2 = (y >> 3) ^ (z << 4);
    let part3 = sum_value ^ y;
    let part4 = k_val ^ z;

    (part1.wrapping_add(part2)) ^ (part3.wrapping_add(part4))
}

fn decrypt_uint32(v: &mut [u32], k: &[u32]) {
    let n = v.len().wrapping_sub(1);
    if n < 1 {
        return;
    }

    let mut sum_value;
    let mut y = v[0];
    let q = 6 + 52 / (n + 1);

    sum_value = (q as u32).wrapping_mul(DELTA);

    while sum_value != 0 {
        let e = (sum_value >> 2) & 3;

        for p in (1..=n).rev() {
            // p goes n down to 1
            let z = v[p - 1];
            let mx_val = mx(sum_value, y, z, p, e, k);
            v[p] = v[p].wrapping_sub(mx_val);
            y = v[p];
        }

        let p = 0;
        let z = v[n];
        let mx_val = mx(sum_value, y, z, p, e, k);
        v[0] = v[0].wrapping_sub(mx_val);
        y = v[0];

        sum_value = sum_value.wrapping_sub(DELTA);
    }
}

pub fn decrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    if data.is_empty() {
        return Ok(Vec::new());
    }
    let fixed_key = fix_key(key);
    let mut v = to_uint32_array(data, false);
    let k = to_uint32_array(&fixed_key, false);

    decrypt_uint32(&mut v, &k);

    to_byte_array(&v, false).ok_or_else(|| "Invalid XXTEA data or key.".to_string())
}
