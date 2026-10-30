use std::process::{Command, Stdio};
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;   // Windows API flag

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
                format!(" Please relaunch the program with administrator rights. Access denied at Index {}: {}", i, code));
        }
        i += 1;
    }
    return Ok(String::from("Stopped all selected running services"));
}

pub fn disable_services(services_codes: &[String]) -> Result<String, String> {
    let mut i = 0;
    for code in services_codes {
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
                format!(" Please relaunch the program with administrator rights. Access denied at Index {}: {}", i, services_codes[i]));
        }
        i += 1;
    }
    return Ok(String::from("Successfully disabled all selected services"));
}

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
    let mut services_desc = Command::new("powershell")
        .args([
            "-Command",
            "Get-CimInstance Win32_Service | Select-Object Description | Format-List"
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .spawn().expect("get service failed");
    let stdout = services_desc.stdout.take().expect("service desc is missing");
    let output = Command::new("findstr")
        .stdin(stdout)
        .args(["Description : "])
        .creation_flags(CREATE_NO_WINDOW)
        .output().expect("cannot filter desc");
    String::from_utf8_lossy(&output.stdout).into_owned().lines()
    .map(|line| {line.replace("Description : ", "").trim().to_string()     
    }).collect()
}
