use tokio::sync::{mpsc, Semaphore};
use tokio::task::JoinHandle;
use std::sync::Arc;
use log::{info, error};
use anyhow::Result;

pub enum WorkerMessage {
    Task(String),
    Stop,
}

pub struct WorkerEngine {
    worker_id: usize,
    concurrency: usize,
    receiver: mpsc::Receiver<WorkerMessage>,
    task_handler: Arc<dyn TaskHandler>,
}

impl WorkerEngine {
    pub fn new(
        worker_id: usize,
        concurrency: usize,
        receiver: mpsc::Receiver<WorkerMessage>,
        task_handler: Arc<dyn TaskHandler>,
    ) -> Self {
        Self {
            worker_id,
            concurrency,
            receiver,
            task_handler,
        }
    }

    pub async fn run(mut self) {
        info!("[Worker-{}] Started with concurrency {}", self.worker_id, self.concurrency);
        
        let semaphore = Arc::new(Semaphore::new(self.concurrency));
        let mut tasks: Vec<JoinHandle<()>> = Vec::new();

        loop {
            match self.receiver.recv().await {
                Some(WorkerMessage::Task(ip)) => {
                    let sem = semaphore.clone();
                    let handler = self.task_handler.clone();
                    let worker_id = self.worker_id;
                    
                    let task = tokio::spawn(async move {
                        let _permit = sem.acquire().await.unwrap();
                        if let Err(e) = handler.handle(ip.clone()).await {
                            error!("[Worker-{}] Error handling {}: {}", worker_id, ip, e);
                        }
                    });
                    
                    tasks.push(task);
                }
                Some(WorkerMessage::Stop) => {
                    break;
                }
                None => {
                    break;
                }
            }
        }

        // منتظر اتمام تمام تسک‌ها
        for task in tasks {
            let _ = task.await;
        }

        info!("[Worker-{}] Finished", self.worker_id);
    }
}

#[async_trait::async_trait]
pub trait TaskHandler: Send + Sync {
    async fn handle(&self, ip: String) -> Result<()>;
}

pub struct WorkerSupervisor {
    sender: mpsc::Sender<WorkerMessage>,
    handle: Option<JoinHandle<()>>,
}

impl WorkerSupervisor {
    pub fn new(
        worker_id: usize,
        concurrency: usize,
        task_handler: Arc<dyn TaskHandler>,
    ) -> Self {
        let (sender, receiver) = mpsc::channel(10000);
        
        let worker = WorkerEngine::new(worker_id, concurrency, receiver, task_handler);
        let handle = tokio::spawn(async move {
            worker.run().await;
        });

        Self {
            sender,
            handle: Some(handle),
        }
    }

    pub async fn send_task(&self, ip: String) -> Result<()> {
        self.sender.send(WorkerMessage::Task(ip)).await?;
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        self.sender.send(WorkerMessage::Stop).await?;
        Ok(())
    }

    pub async fn wait(mut self) {
        if let Some(handle) = self.handle.take() {
            let _ = handle.await;
        }
    }
}
