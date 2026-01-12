import time

class Timer:
    def __init__(self):
        self.start_time = None
        self.end_time = None

    def start(self):
        self.start_time = time.time()

    def stop(self):
        if self.start_time is None:
            raise Exception("Timer hasn't started yet!")
        self.end_time = time.time()

    def get_elapsed_time(self):
        if self.start_time is None or self.end_time is None:
            raise Exception("Timer hasn't been started and stopped properly!")
        elapsed_time = self.end_time - self.start_time
        return self.format_time(elapsed_time)

    def format_time(self, seconds):
        hours = int(seconds // 3600)
        minutes = int((seconds % 3600) // 60)
        seconds = int(seconds % 60)
        return f"{hours} hours, {minutes} minutes, {seconds} seconds"

if "__name__" == "__main__":
    timer = Timer()
    timer.start()
    # اینجا کد پروژه شما قرار می‌گیره
    for i in range(1000000):
        pass
    timer.stop()
    # نمایش زمان
    print(f"زمان کل پروژه: {timer.get_elapsed_time()}")