#[cfg(feature = "debug")]
use colored::*;

#[cfg(feature = "debug")]
pub fn print_error(_prefix: &str, _error: &dyn std::fmt::Display) {
    println!("{} {}", _prefix.red(), _error);
}

#[cfg(feature = "debug")]
pub fn print_message(msg: &str) {
    println!("{}",msg.green());
}

#[allow(dead_code)]
pub fn simple_decrypt(encrypted: &str) -> String {
    use rustcrypt_ct_macros::obf_lit_bytes;
    use base64::{Engine as _, engine::general_purpose};
    let decoded = general_purpose::STANDARD.decode(encrypted).unwrap();
    let obf_key = obf_lit_bytes!(b"rsl_secret_key_2025");
    let key = obf_key.as_slice();
    let decrypted: Vec<u8> = decoded.iter().enumerate().map(|(i, &b)| b ^ key[i % key.len()]).collect();
    String::from_utf8(decrypted).unwrap()
}

#[allow(dead_code)]
pub unsafe fn load_library(dll_name: &[u8]) -> Result<isize, String> {
    use windows_sys::Win32::System::LibraryLoader::LoadLibraryA;
    use rustcrypt_ct_macros::obf_lit;
    let dll = LoadLibraryA(dll_name.as_ptr() as *const u8);
    if dll == 0 {
        Err(obf_lit!("LoadLibraryA failed").to_string())
    } else {
        Ok(dll)
    }
}

#[allow(dead_code)]
pub unsafe fn get_proc_address(dll: isize, name: &[u8]) -> Result<*const (), String> {
    use windows_sys::Win32::System::LibraryLoader::GetProcAddress;
    use rustcrypt_ct_macros::obf_lit;
    let addr = GetProcAddress(dll, name.as_ptr() as *const u8);
    if let Some(f) = addr {
        Ok(f as *const ())
    } else {
        Err(obf_lit!("GetProcAddress failed").to_string())
    }
}

#[allow(dead_code)]
pub fn obfuscation_noise() {
    use std::collections::HashMap;
    use rustcrypt_ct_macros::obf_lit_bytes;
    use rand::Rng;
    
    let mut rng = rand::thread_rng();
    
    let mut hash_map: HashMap<i32, String> = HashMap::new();
    let map_size = rng.gen_range(5..15);
    for _ in 0..map_size {
        let key = rng.gen::<i32>();
        let val = format!("value_{}", rng.gen::<u32>());
        hash_map.insert(key, val);
    }
    
    let mut sum: u64 = 0;
    let sum_iterations = rng.gen_range(500..1500);
    for i in 0..sum_iterations {
        sum = sum.wrapping_add((i as u64).wrapping_mul(rng.gen::<u64>()));
    }
    
    let test_str = obf_lit_bytes!(b"random_buffer_data");
    let mut buffer: Vec<u8> = test_str.iter().map(|&b| b.wrapping_add(rng.gen::<u8>())).collect();
    if rng.gen_bool(0.5) {
        buffer.reverse();
    }
    let _ = buffer.len();
    
    let filter_mod = rng.gen_range(2..5);
    let _result: Vec<i32> = (0..rng.gen_range(50..150))
        .filter(|x| x % filter_mod == 0)
        .map(|x| x * x)
        .take(rng.gen_range(10..30))
        .collect();
    
    use std::time::Instant;
    let _start = Instant::now();
    let loop_count = rng.gen_range(50000..150000);
    for _ in 0..loop_count {
        let _ = (rng.gen::<i32>()).wrapping_mul(rng.gen::<i32>());
    }
    
    let mut val: u32 = rng.gen::<u32>();
    let shift_count = rng.gen_range(5..10);
    for _ in 0..shift_count {
        val = val.wrapping_shl(1) ^ val.wrapping_shr(3);
    }
    let _ = val;
    
    for (k, v) in hash_map.iter() {
        let _ = format!("{}={}", k, v);
    }
    
    let sum_range = rng.gen_range(20..80);
    let _ = (0..sum_range).map(|x| x * x).sum::<i32>();
}
