import threading
import time
from .logger import logger

class WorkerSupervisor:
    def __init__(self, wid, concurrency, worker_class):
        self.wid = wid
        self.concurrency = concurrency
        self.worker_class = worker_class
        self.worker = None
        self.thread = None
        self.start()

    def start(self):
        self.worker = self.worker_class(self.wid, self.concurrency)
        self.thread = threading.Thread(
            target=self.worker.run,
            name=f"Worker-{self.wid}",
            daemon=True
        )
        self.thread.start()
        logger.info(f"[Supervisor] Worker-{self.wid} started")

    def is_alive(self):
        return self.thread.is_alive()

    def restart(self):
        logger.info(f"[Supervisor] Restarting Worker-{self.wid}")
        self.start()

def watchdog_monitor(supervisors, stop_event):
    while not stop_event.is_set():
        for s in supervisors:
            if not s.is_alive():
                s.restart()
        time.sleep(1)