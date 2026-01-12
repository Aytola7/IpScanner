import asyncio
from .logger import logger
from abc import ABC, abstractmethod
from queue import Queue

STOP = object()

class BaseWorker(ABC):
    def __init__(self, wid: int, concurrency: int):
        self.wid = wid
        self.concurrency = concurrency
        self.queue = Queue()

    def get_queue_size(self):
        return self.queue.qsize()

    @abstractmethod
    async def handle(self, item):
        pass

    async def consumer(self, sem):
        try:
            while True:
                item = await asyncio.to_thread(self.queue.get)
                if item is STOP:
                    self.queue.task_done()
                    break
                async with sem:
                    try:
                        #logger.info(f'{self.get_queue_size()} tasks remaining in Worker-{self.wid}')
                        await self.handle(item)
                    except Exception as e:
                        logger.error(f"[Worker-{self.wid}] error on {item}: {e}")
                    finally:
                        self.queue.task_done()
        except Exception as ex:
            logger.error(ex)
        return

    async def _run_async(self):
        sem = asyncio.Semaphore(self.concurrency)
        tasks = [asyncio.create_task(self.consumer(sem)) for _ in range(self.concurrency)]
        await asyncio.gather(*tasks)

    def run(self):
        loop = asyncio.new_event_loop()
        asyncio.set_event_loop(loop)
        try:
            loop.run_until_complete(self._run_async())
        finally:
            loop.close()