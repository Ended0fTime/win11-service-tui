use crate::{App};
use windows_service::{
    service::{Service, ServiceAccess, ServiceStartType, ServiceState}, service_manager::{ServiceManager, ServiceManagerAccess},
};
use std::process::{Command, Stdio};
use std::os::windows::process::CommandExt;
use color_eyre::{
    Result, eyre::WrapErr,
};


const CREATE_NO_WINDOW: u32 = 0x08000000;   // Windows API flag


impl App {
    // fn call_service_manager(&self) -> ServiceManager {
    //     let access = ServiceManagerAccess::CONNECT;
    //     let manager = ServiceManager::local_computer(None::<&str>, access);
    //     return manager.expect("Something in call_service_manager went wrong");
    // }

    pub fn get_services_name_short(&self) -> Vec<String> {
        let mut services_name = Command::new("sc")
            .args(["query", "state=all"])
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(Stdio::piped())
            .spawn().unwrap();
        let stdout = services_name.stdout.take().unwrap();
        let output = Command::new("findstr")
            .stdin(stdout)
            .args(["SERVICE_NAME"])
            .creation_flags(CREATE_NO_WINDOW)
            .output().unwrap();
        String::from_utf8_lossy(&output.stdout).into_owned().lines()
        .map(|line| {line.replace("SERVICE_NAME:", "").trim().to_string()     
        }).collect()
    }

    pub fn get_services_name(&self) -> Vec<String> {
        let mut services_name = Command::new("sc")
            .args(["query", "state=all"])
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(Stdio::piped())
            .spawn().unwrap();
        let stdout = services_name.stdout.take().unwrap();
        let output = Command::new("findstr")
            .stdin(stdout)
            .args(["DISPLAY_NAME"])
            .creation_flags(CREATE_NO_WINDOW)
            .output().unwrap();
        String::from_utf8_lossy(&output.stdout).into_owned().lines()
        .map(|line| {line.replace("DISPLAY_NAME:", "").trim().to_string()     
        }).collect()
    }

    pub fn get_services_desc(&self) -> Vec<String> {
        let mut services_desc = Command::new("powershell")
            .args([
                "-Command",
                "Get-CimInstance Win32_Service | Select-Object Description | Format-List"
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(Stdio::piped())
            .spawn().unwrap();
        let stdout = services_desc.stdout.take().unwrap();
        let output = Command::new("findstr")
            .stdin(stdout)
            .args(["Description : "])
            .creation_flags(CREATE_NO_WINDOW)
            .output().unwrap();
        String::from_utf8_lossy(&output.stdout).into_owned().lines()
        .map(|line| {line.replace("Description : ", "").trim().to_string()     
        }).collect()
    }

    pub fn disable_services(&mut self) {
        self.cmd_output_stop.clear();    // clear for new message

        let mut i = 0;
        for code in &self.services_code_selected {
            let output_stop = Command::new("sc")
                .arg("stop")
                .arg(code)
                .creation_flags(CREATE_NO_WINDOW)
                .output().unwrap();
            let output_disable = Command::new("sc")
                .arg("config")
                .arg(code)
                .arg("start=")
                .arg("disabled")
                .creation_flags(CREATE_NO_WINDOW)
                .output().unwrap();
            
            let stdout = String::from_utf8_lossy(&output_stop.stdout);
            if stdout.contains("FAILED") && !stdout.contains("1062") {
                self.cmd_output_stop.push_str(&format!(" Please relaunch the program with administrator rights. Access denied at Index {}: {}", i, &self.services_selected[i]));
                break;
            }
            else {
                self.cmd_output_stop = String::from("Stopped all running services");
            }

            let stdout_disable = String::from_utf8_lossy(&output_disable.stdout);
            self.cmd_output_disable.push_str(&stdout_disable.to_string());
            if stdout_disable.contains("FAILED") {
                self.cmd_output_disable.push_str(&format!(" Please relaunch the program with administrator rights. Access denied at Index {}: {}", i, &self.services_selected[i]));
                break;
            }
            else {
                self.cmd_output_disable.clear();
                self.cmd_output_disable = String::from(" Successfully disabled selected services.");
            }
            i = i + 1;
        }
    }

}