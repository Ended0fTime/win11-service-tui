use std::process::Command;
use std::os::windows::process::CommandExt;

#[derive(Default)]
pub struct ServiceInfo {
    pub names: String,
    pub codes: String,
    pub start_type: String,
    pub desc: String,
}

const CREATE_NO_WINDOW: u32 = 0x08000000;   // Windows API flag


pub fn get_service_metadata() -> Vec<ServiceInfo> {
    let mut services = Vec::new();
    let script = "Get-CimInstance Win32_Service | ForEach-Object { $_.DisplayName + ':::' + $_.Name + ':::' + $_.StartMode + ':::' + $_.Description }";

    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", script])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .expect("Failed to get service metadata");

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        // splitn(5, ...) ensures descriptions containing ':::' won't break earlier fields
        let parts: Vec<&str> = line.splitn(5, ":::").collect();
        if parts.len() >= 4 {
            services.push(ServiceInfo {
                names: parts[0].trim().to_string(),
                codes: parts[1].trim().to_string(),
                start_type: parts[2].trim().to_string(),
                desc: if parts.len() == 4 { parts[3].trim().to_string() } else { "No description given".to_string() },
            });
        }
    }
    services

}

pub fn update_service_start_type(service: &String) -> String {
    let output = Command::new("sc")
        .args(["qc", service])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    let Ok(output) = output else { return "Unknown".to_string(); };
    let stdout = String::from_utf8_lossy(&output.stdout);

    let raw_token = stdout
        .lines()
        .find(|line| line.contains("START_TYPE"))
        .and_then(|line| line.split_whitespace().last())
        .unwrap_or("UNKNOWN");

    match raw_token.to_uppercase().as_str() {
        "AUTO_START" => "Automatic",
        "(DELAYED)" => "Auto (Delayed)",
        "DEMAND_START" => "Manual",
        "DISABLED" => "Disabled",
        "BOOT_START" => "Boot",
        "SYSTEM_START" => "System",
        _ => "Unknown",
    }.to_string()
}

pub fn get_services_desc() -> Vec<String> {
    let services_desc = Command::new("powershell")
        .args([
            "-Command",
            "Get-CimInstance Win32_Service | ForEach-Object { $_.Description }"
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .expect("get service failed");
    let output = services_desc.stdout;
    String::from_utf8_lossy(&output)
    .lines()
    .map(|line| {line.trim().to_string()})
    .collect()
}


pub fn stop_services(services_codes: &[String]) -> Result<String, String> {
    for (i, code) in services_codes.iter().enumerate() {
        let output_stop = Command::new("sc")
            .arg("stop")
            .arg(code)
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .expect("couldnt stop FATAL");             
        let stdout = String::from_utf8_lossy(&output_stop.stdout);
        if stdout.contains("FAILED") && !stdout.contains("1062") {
            return Err(
                format!(" {} at Index {}: {}", stdout, i, services_codes[i]));
        }
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
            .output()
            .expect("couldnt disable FATAL");

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
            .output()
            .expect("couldnt set to manual FATAL");

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
            .output()
            .expect("couldnt set to auto FATAL");

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
            .output()
            .expect("couldnt set to auto (delayed) FATAL");

        let stdout = String::from_utf8_lossy(&output_disable.stdout);
        if stdout.contains("FAILED") && !stdout.contains("1056") {
            return Err(
                format!(" {} at Index {}: {}", stdout, i, services_codes[i]));
        }
    }
    Ok(String::from("Successfully set all selected services to auto (delayed)"))
}

