use std::process::{Command, Stdio};
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;   // Windows API flag


pub fn get_services_name_short() -> Vec<String> {
    let services_name = Command::new("sc")
        .args(["query", "state=all"])
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .spawn().expect("code ccooked");
    let stdout = services_name.stdout.expect("Service code missing");
    let output = Command::new("findstr")
        .stdin(stdout)
        .args(["SERVICE_NAME"])
        .creation_flags(CREATE_NO_WINDOW)
        .output().expect("code cooked");
    String::from_utf8_lossy(&output.stdout).into_owned().lines()
    .map(|line| {line.replace("SERVICE_NAME:", "").trim().to_string()     
    }).collect()
}

pub fn get_services_name() -> Vec<String> {
    let mut services_name = Command::new("sc")
        .args(["query", "state=all"])
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .spawn().expect("name cooked");
    let stdout = services_name.stdout.take().expect("Service name missing");
    let output = Command::new("findstr")
        .stdin(stdout)
        .args(["DISPLAY_NAME"])
        .creation_flags(CREATE_NO_WINDOW)
        .output().expect("name cooked");
    String::from_utf8_lossy(&output.stdout).into_owned().lines()
    .map(|line| {line.replace("DISPLAY_NAME:", "").trim().to_string()     
    }).collect()
}

pub fn get_services_desc() -> Vec<String> {
    let services_desc = Command::new("powershell")
        .args([
            "-Command",
            "Get-CimInstance Win32_Service | Select-Object -ExpandProperty Description"
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output().expect("get service failed");
    let output = services_desc.stdout;
    String::from_utf8_lossy(&output).into_owned().lines()
    .map(|line| {line.trim().to_string()     
    }).collect()
}


pub fn stop_services(services_codes: &[String]) -> Result<String, String> {
    let mut i = 0;
    for code in services_codes {
        let output_stop = Command::new("sc")
            .arg("stop")
            .arg(code)
            .creation_flags(CREATE_NO_WINDOW)
            .output().expect("couldnt stop FATAL");             
        let stdout = String::from_utf8_lossy(&output_stop.stdout);
        if stdout.contains("FAILED") && !stdout.contains("1062") {
            return Err(
                format!(" {} at Index {}: {}", stdout, i, services_codes[i]));
        }
        i += 1;
    }
    Ok(String::from("Stopped all selected running services"))
}

pub fn disable_services(services_codes: &[String]) -> Result<String, String> {
    for (i, code) in services_codes.iter().enumerate() {
        let output_disable = Command::new("sc")
            .arg("config")
            .arg(code)
            .arg("start=")
            .arg("disabled")
            .creation_flags(CREATE_NO_WINDOW)
            .output().expect("couldnt disable FATAL");

        let stdout = String::from_utf8_lossy(&output_disable.stdout);
        if stdout.contains("FAILED") {
            return Err(
                format!(" {} at Index {}: {}", stdout, i, services_codes[i]));
        }
    }
    Ok(String::from("Successfully disabled all selected services"))
}

pub fn enable_manual(services_codes: &[String]) -> Result<String, String> {
    for (i, code) in services_codes.iter().enumerate() {
        let output_disable = Command::new("sc")
            .arg("config")
            .arg(code)
            .arg("start=")
            .arg("demand")
            .creation_flags(CREATE_NO_WINDOW)
            .output().expect("couldnt set to manual FATAL");

        let stdout = String::from_utf8_lossy(&output_disable.stdout);
        if stdout.contains("FAILED") {
            return Err(
                format!(" {} at Index {}: {}", stdout, i, services_codes[i]));
        }
    }
    Ok(String::from("Successfully set all selected services to manually triggered"))
}

pub fn enable_auto(services_codes: &[String]) -> Result<String, String> {
    for (i, code) in services_codes.iter().enumerate() {
        let output_disable = Command::new("sc")
            .arg("config")
            .arg(code)
            .arg("start=")
            .arg("auto")
            .creation_flags(CREATE_NO_WINDOW)
            .output().expect("couldnt set to auto FATAL");

        let stdout = String::from_utf8_lossy(&output_disable.stdout);
        if stdout.contains("FAILED") && !stdout.contains("1056") {
            return Err(
                format!(" {} at Index {}: {}", stdout, i, services_codes[i]));
        }
    }
    Ok(String::from("Successfully set all selected services to auto"))
}

pub fn enable_auto_delayed(services_codes: &[String]) -> Result<String, String> {
    for (i, code) in services_codes.iter().enumerate() {
        let output_disable = Command::new("sc")
            .arg("config")
            .arg(code)
            .arg("start=")
            .arg("delayed-auto")
            .creation_flags(CREATE_NO_WINDOW)
            .output().expect("couldnt set to auto (delayed) FATAL");

        let stdout = String::from_utf8_lossy(&output_disable.stdout);
        if stdout.contains("FAILED") && !stdout.contains("1056") {
            return Err(
                format!(" {} at Index {}: {}", stdout, i, services_codes[i]));
        }
    }
    Ok(String::from("Successfully set all selected services to auto (delayed)"))
}

