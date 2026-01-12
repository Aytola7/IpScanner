use std::fs;
use std::io::{self, Write};
use std::net::Ipv4Addr;
use std::path::Path;
use ipnetwork::Ipv4Network;
use rand::seq::SliceRandom;
use anyhow::{Result, Context};

#[derive(Debug, Clone)]
pub struct IpGroup {
    pub label: String,
    pub ips: Vec<String>,
}

pub fn create_ips(ip_file: &Path) -> Result<Vec<IpGroup>> {
    // خواندن فایل IP
    let content = fs::read_to_string(ip_file)
        .context("Failed to read IP file")?;
    
    let ip_lines: Vec<String> = content
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();

    let mut groups_raw = Vec::new();

    for line in ip_lines {
        match expand_ip_line(&line) {
            Ok(ips) => {
                if !ips.is_empty() {
                    groups_raw.push(IpGroup {
                        label: line.clone(),
                        ips,
                    });
                }
            }
            Err(e) => {
                eprintln!("Warning: couldn't parse IP line '{}': {}", line, e);
            }
        }
    }

    // درخواست درصد از کاربر
    print!("What percent of IPs to test? (0-100) [100]: ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let percent: u32 = input.trim().parse().unwrap_or(100).min(100);

    // نمونه‌گیری
    let mut final_groups = Vec::new();
    let mut rng = rand::thread_rng();

    for group in groups_raw {
        let total = group.ips.len();
        if total == 0 {
            continue;
        }

        let sampled = if percent >= 100 {
            group.ips
        } else {
            let k = ((total as f64 * (percent as f64 / 100.0)).ceil() as usize).max(1).min(total);
            let mut ips = group.ips;
            ips.partial_shuffle(&mut rng, k).0.to_vec()
        };

        final_groups.push(IpGroup {
            label: format!("{} (sample {}%)", group.label, percent),
            ips: sampled,
        });
    }

    Ok(final_groups)
}

fn expand_ip_line(line: &str) -> Result<Vec<String>> {
    let line = line.trim();
    
    if line.is_empty() {
        return Ok(Vec::new());
    }

    // بررسی CIDR notation (مثل 192.168.1.0/24)
    if line.contains('/') {
        return expand_cidr(line);
    }

    // بررسی range (مثل 192.168.1.1-192.168.1.254)
    if line.contains('-') {
        return expand_range(line);
    }

    // IP تکی
    line.parse::<Ipv4Addr>()?;
    Ok(vec![line.to_string()])
}

fn expand_cidr(cidr: &str) -> Result<Vec<String>> {
    let network: Ipv4Network = cidr.parse()?;
    
    let mut ips = Vec::new();
    for ip in network.iter() {
        // حذف network و broadcast address
        if ip != network.network() && ip != network.broadcast() {
            ips.push(ip.to_string());
        }
    }
    
    // اگر فقط یک IP داریم (مثل /32)
    if ips.is_empty() && network.size() == 1 {
        ips.push(network.network().to_string());
    }
    
    Ok(ips)
}

fn expand_range(range: &str) -> Result<Vec<String>> {
    let parts: Vec<&str> = range.split('-').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid range format");
    }

    let start_ip: Ipv4Addr = parts[0].trim().parse()?;
    let end_ip: Ipv4Addr = parts[1].trim().parse()?;

    let start = u32::from(start_ip);
    let end = u32::from(end_ip);

    if end < start {
        anyhow::bail!("End IP is less than start IP");
    }

    const MAX_RANGE: u32 = 65536;
    if end - start > MAX_RANGE {
        anyhow::bail!("Range too large (max 65536)");
    }

    let mut ips = Vec::new();
    for ip_int in start..=end {
        ips.push(Ipv4Addr::from(ip_int).to_string());
    }

    Ok(ips)
}
