mod ip_creator;
mod worker_engine;
mod task_handler;
mod timer;

use clap::Parser;
use log::info;
use std::path::PathBuf;
use std::sync::Arc;
use anyhow::Result;

use ip_creator::create_ips;
use worker_engine::WorkerSupervisor;
use task_handler::AppTaskHandler;
use timer::Timer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of workers
    #[arg(short, long, default_value_t = 100)]
    workers: usize,

    /// Concurrency per worker
    #[arg(short, long, default_value_t = 30)]
    concurrency: usize,

    /// IP file path
    #[arg(short, long, default_value = "ip4.txt")]
    ip_file: PathBuf,

    /// Output file for successful pings
    #[arg(long, default_value = "safePing.txt")]
    safe_ping: String,

    /// Output file for successful socket connections
    #[arg(long, default_value = "safeSocketConnect.txt")]
    safe_socket: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // راه‌اندازی logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();

    let args = Args::parse();

    info!("Starting IP Scanner");
    info!("Workers: {}, Concurrency per worker: {}", args.workers, args.concurrency);

    // شروع تایمر
    let mut timer = Timer::new();
    timer.start();

    // خواندن و پردازش IP ها
    let ip_groups = create_ips(&args.ip_file)?;
    
    let total_ips: usize = ip_groups.iter().map(|g| g.ips.len()).sum();
    info!("Total IPs to scan: {}", total_ips);

    // ساخت task handler
    let task_handler = Arc::new(AppTaskHandler::new(
        args.safe_ping.clone(),
        args.safe_socket.clone(),
    ));

    // ساخت worker supervisors
    let mut supervisors = Vec::new();
    for i in 1..=args.workers {
        let supervisor = WorkerSupervisor::new(i, args.concurrency, task_handler.clone());
        supervisors.push(supervisor);
    }

    info!("All workers started");

    // توزیع تسک‌ها بین workerها
    let mut i = 0;
    for group in ip_groups {
        info!("Processing group: {}", group.label);
        for ip in group.ips {
            let supervisor = &supervisors[i % supervisors.len()];
            supervisor.send_task(ip).await?;
            i += 1;
        }
    }

    info!("All tasks distributed");

    // ارسال سیگنال Stop به همه workerها
    for supervisor in &supervisors {
        supervisor.stop().await?;
    }

    info!("Waiting for all workers to finish...");

    // منتظر اتمام همه workerها
    for supervisor in supervisors {
        supervisor.wait().await;
    }

    // توقف تایمر
    timer.stop();
    
    info!("Finished. Total Time: {}", timer.get_elapsed_time());

    Ok(())
}
