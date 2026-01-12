import logging
import sys

def setup_logger(name=__name__, log_file="app.log"):
    """
    Create and configure a logger.

    :param name: The name of the logger (defaults to the current module name).
    :param log_file: The path to the file to store the logs (defaults to 'app.log').
    :return: A configured logger object.
    # main.py
    
    from logger import logger
    def main():
        logger.debug("This is a debug message.")
        logger.info("This is an informational message.")
        logger.warning("This is a warning message.")
        logger.error("This is an error message.")
        logger.critical("This is a critical message.")
    if __name__ == "__main__":
        main()

    # utils.py
    from logger import logger

    def do_something():
        logger.info("Doing something...")
    try:
        #Your code
        pass
    except Exception as e:
        logger.error(f"Error doing task: {e}")
    """
    logger = logging.getLogger(name)
    if logger.hasHandlers():
        return logger
    logger.setLevel(logging.DEBUG)
    formatter = logging.Formatter("%(asctime)s - %(name)s - %(levelname)s - %(message)s")
    try:
        file_handler = logging.FileHandler(log_file)
        file_handler.setFormatter(formatter)
        logger.addHandler(file_handler)
    except Exception as e:
        print(f"Error creating FileHandler: {e}", file=sys.stderr)
    stream_handler = logging.StreamHandler(sys.stdout)
    stream_handler.setFormatter(formatter)
    logger.addHandler(stream_handler)
    return logger

logger = setup_logger(__name__)