use std::path::{Path, PathBuf};
use sysinfo::System;
use uname::uname;
use serde::Serialize;
use whoami::fallible;

#[derive(Serialize)]
pub struct SystemInfo {
    os: String,
    kernel: String,
    uptime: String,
    hostname: String,
    cpu: String,
    memory: String,
    user: String,
}

impl SystemInfo {
    pub fn new() -> Self {
        let sys = System::new_all();
        let uname_info = uname().unwrap();
        
        let os = System::long_os_version().unwrap_or_else(|| "Unknown OS".to_string());
        let kernel = uname_info.release;
        let uptime = format!("{} seconds", sysinfo::System::uptime());
        let hostname = fallible::hostname().unwrap_or("unknown".to_string());
        let cpu = sys
            .cpus()
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| "Unknown CPU".to_string());
        let memory = format!("{:.2} GB", sys.total_memory() as f64 / (1024.0 * 1024.0));
        let user = whoami::username();

        SystemInfo {
            os,
            kernel,
            uptime,
            hostname,
            cpu,
            memory,
            user,
        }
    }
}

