use crate::worker_engine::TaskHandler;
use anyhow::Result;
use log::{info, error};
use std::fs::OpenOptions;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::process::Command;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use rand::Rng;

pub struct AppTaskHandler {
    safe_ping_file: String,
    safe_socket_file: String,
}

impl AppTaskHandler {
    pub fn new(safe_ping_file: String, safe_socket_file: String) -> Self {
        Self {
            safe_ping_file,
            safe_socket_file,
        }
    }

    async fn ping_test(&self, ip: String) -> Result<()> {
        // کمی تاخیر تصادفی
        let delay = rand::thread_rng().gen_range(0..1000);
        sleep(Duration::from_millis(delay)).await;

        // اجرای دستور ping (Linux)
        let output = tokio::task::spawn_blocking({
            let ip = ip.clone();
            move || {
                Command::new("ping")
                    .arg("-c")
                    .arg("2")
                    .arg("-W")
                    .arg("5")
                    .arg(&ip)
                    .output()
            }
        })
        .await??;

        if output.status.success() {
            // ذخیره IP موفق
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.safe_ping_file)
            {
                writeln!(file, "{}", ip)?;
            }
            info!("IP is available (ping): {}", ip);
        }

        Ok(())
    }

    async fn socket_test(&self, ip: String) -> Result<()> {
        // کمی تاخیر تصادفی
        let delay = rand::thread_rng().gen_range(0..1000);
        sleep(Duration::from_millis(delay)).await;

        let ports = vec![13, 22, 23, 80, 443, 3389];
        
        for port in ports {
            let addr = format!("{}:{}", ip, port);
            
            // تست اتصال TCP
            let result = tokio::task::spawn_blocking({
                let addr = addr.clone();
                move || {
                    if let Ok(parsed_addr) = addr.parse::<SocketAddr>() {
                        TcpStream::connect_timeout(&parsed_addr, Duration::from_secs(1))
                    } else {
                        Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid address"))
                    }
                }
            })
            .await?;

            if result.is_ok() {
                // ذخیره IP:Port موفق
                if let Ok(mut file) = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&self.safe_socket_file)
                {
                    writeln!(file, "{}:{}", ip, port)?;
                }
                info!("IP is available (socket): {}:{}", ip, port);
            }
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl TaskHandler for AppTaskHandler {
    async fn handle(&self, ip: String) -> Result<()> {
        let start = Instant::now();
        
        // اجرای همزمان هر دو تست
        let ping_future = self.ping_test(ip.clone());
        let socket_future = self.socket_test(ip.clone());
        
        let (ping_result, socket_result) = tokio::join!(ping_future, socket_future);
        
        if let Err(e) = ping_result {
            error!("Ping test error for {}: {}", ip, e);
        }
        
        if let Err(e) = socket_result {
            error!("Socket test error for {}: {}", ip, e);
        }
        
        let elapsed = start.elapsed();
        info!("Task-{} completed in {:.2}s", ip, elapsed.as_secs_f64());
        
        Ok(())
    }
}
