import argparse
import threading
from worker.timer import Timer
from worker.logger import logger
from worker import WorkerSupervisor, watchdog_monitor
from worker.task_handler import AppTaskHandler
from worker import STOP
from worker.createip import createip

def main():
    handler_type = AppTaskHandler
    parser = argparse.ArgumentParser()
    parser.add_argument("--workers", type=int, default=100)
    parser.add_argument("--concurrency", type=int, default=30)
    args = parser.parse_args()
    timer = Timer()
    timer.start()
    stop_event = threading.Event()
    supervisors = [
        WorkerSupervisor(i, args.concurrency, handler_type)
        for i in range(1, args.workers + 1)
    ]
    wd = threading.Thread(target=watchdog_monitor, args=(supervisors, stop_event), daemon=True)
    wd.start()
    i = 1
    for ips in createip(): # تعداد تسک تستی
        for ip in ips.get("ips",[]):
            s = supervisors[i % len(supervisors)]
            s.worker.queue.put(f"{ip}")
            i += 1
    # انتظار برای اتمام
    for s in supervisors:
        s.worker.queue.join()
    # خاموشی منظم
    for s in supervisors:
        for _ in range(s.concurrency):
            s.worker.queue.put(STOP)
    stop_event.set()
    timer.stop()
    logger.info(f"Finished. Total Time: {timer.get_elapsed_time()}")

if __name__ == "__main__":
    main()