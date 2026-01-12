import time
from .logger import logger
from worker import BaseWorker
import asyncio
import socket
import os
import random

class AppTaskHandler(BaseWorker):
    async def handle(self, item):
        start = time.time()
        async def pingtest(item):
            await asyncio.sleep(random.random())
            try:
                result =  os.system(f"ping -w 5 "+ item)
                if result == 0:
                    with open("safePing.txt",'a+',encoding='UTF-8') as fe:
                        fe.write(f"{item}\n")
                    logger.info(f"Ip Is Available ping: {item}")
                return 
            except Exception as ex:
                logger.error(f"{ex}")

        async def pingtestsocket(item):
            await asyncio.sleep(random.random())
            s = socket.socket(socket.AF_INET,socket.SOCK_STREAM)
            s.settimeout(1)
            for port in [13,22,23,80,443,3389]:
                try:
                    response = s.connect_ex((item,port))
                    if response == 0:
                        with open("safeSocketConnect.txt",'a+',encoding='UTF-8') as fe:
                            fe.write(f"{item}:{port}\n")
                        logger.info(f"Ip Is Available Socket: {item}:{port}")
                except Exception as ex:
                    logger.error(f"{ex}")
            return 
        try:
            task = asyncio.create_task(pingtestsocket(item))
            task2 = asyncio.create_task(pingtest(item))
            await asyncio.gather(task,task2)
        except Exception as ex:
            logger.error(f"{ex}")
        end = time.time()
        logger.info(f"Task-{item} completed in {end - start:.2f}s")
        return
        
