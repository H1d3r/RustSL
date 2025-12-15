use obfstr::obfstr;
use std::rc::Rc;

#[allow(dead_code)]
fn get_wmi_connection() -> Option<wmi::WMIConnection> {
    use wmi::{WMIConnection, COMLibrary};

    let com_con = COMLibrary::new().ok()?;
    WMIConnection::with_namespace_path(&obfstr!("root\\cimv2"), Rc::new(com_con)).ok()
}

#[allow(dead_code)]
pub fn check_cpu_model() -> bool {
    use wmi::Variant;

    let wmi_con = match get_wmi_connection() {
        Some(con) => con,
        None => return false,
    };

    let results: Vec<std::collections::HashMap<String, Variant>> = match wmi_con.raw_query(obfstr!("SELECT Name FROM Win32_Processor")) {
        Ok(res) => res,
        Err(_) => return false,
    };

    if let Some(proc) = results.get(0) {
        if let Some(Variant::String(name)) = proc.get("Name") {
            let name_lower = name.to_lowercase();
            if name_lower.contains(&obfstr!("qemu")) || name_lower.contains(&obfstr!("virtualbox")) || name_lower.contains(&obfstr!("vmware")) || name_lower.contains(&obfstr!("xen")) {
                return true; // Detected VM
            }
        }
    }

    false
}

#[allow(dead_code)]
pub fn check_cpu_cores(min_cores: u32) -> bool {
    use wmi::Variant;

    let wmi_con = match get_wmi_connection() {
        Some(con) => con,
        None => return false,
    };

    let results: Vec<std::collections::HashMap<String, Variant>> = match wmi_con.raw_query(obfstr!("SELECT NumberOfCores FROM Win32_Processor")) {
        Ok(res) => res,
        Err(_) => return false,
    };

    if let Some(proc) = results.get(0) {
        if let Some(Variant::UI4(cores)) = proc.get("NumberOfCores") {
            if *cores < min_cores {
                return true; // Low cores, likely VM
            }
        }
    }

    false
}

#[allow(dead_code)]
pub fn check_cpu_vendor() -> bool {
    use wmi::Variant;

    let wmi_con = match get_wmi_connection() {
        Some(con) => con,
        None => return false,
    };

    let results: Vec<std::collections::HashMap<String, Variant>> = match wmi_con.raw_query(obfstr!("SELECT Manufacturer FROM Win32_Processor")) {
        Ok(res) => res,
        Err(_) => return false,
    };

    if let Some(proc) = results.get(0) {
        if let Some(Variant::String(manufacturer)) = proc.get("Manufacturer") {
            let man_lower = manufacturer.to_lowercase();
            if man_lower.contains(&obfstr!("vboxvboxvbox")) || man_lower.contains(&obfstr!("vmware")) || man_lower.contains(&obfstr!("qemu")) {
                return true; // Detected VM vendor
            }
        }
    }

    false
}