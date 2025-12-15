#[allow(dead_code)]
use obfstr::obfstr;
pub fn has_usb_history() -> bool {
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};
    use winreg::RegKey;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    
    // Check USBSTOR registry key for subkeys (indicating USB storage devices were connected)
    if let Ok(usbstor) = hklm.open_subkey_with_flags(obfstr!("SYSTEM\\CurrentControlSet\\Enum\\USBSTOR"), KEY_READ) {
        let subkeys: Vec<String> = usbstor.enum_keys().filter_map(|x| x.ok()).collect();
        if !subkeys.is_empty() {
            return true;
        }
    }
    
    false
}