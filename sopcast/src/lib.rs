pub mod config;
use crate::config::Config;
use std::error::Error;
use std::process::{Command, Stdio};


pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let wine = "wine";
    let sopcast = home_dir() + "/.wine/drive_c/Program Files (x86)/SopCast/SopCast.exe";
    let channel = config.get_channel();
    let sopurl = format!("sop://broker.sopcast.com:3912/{}", channel);
    println!("Streaming SopCast channel {}...", channel);
    let mut exited_cleanly = false;
    while !exited_cleanly {
        let status = Command::new(wine)
            .args(&[&sopcast, &sopurl])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;
        if status.success() { exited_cleanly = true };
    }
    Ok(())
}

fn home_dir() -> String {
    // dirs is an external crate that implements the most well accepted OS path functions.
    // The call to dirs::home_dir() returns an std::path::PathBuf, an owned mutable StringBuilder type of Path
    // that complements the borrowed Path type that works on static strings.
    // The function can fail, so it must be unwrapped to get to the PathBuf.
    // The only data member in PathBuf is an OsString because in Windows not all paths are valid UTF8,
    // which is the default encoding of Rust's String.  That's why the .into_string() conversion
    // might also fail, which means it too has to be unwrapped.
    dirs::home_dir().unwrap().into_os_string().into_string().unwrap()
}