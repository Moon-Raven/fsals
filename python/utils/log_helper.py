"""This module initializes and partially provides logging facilities."""
import logging
import sys
from pathlib import Path
import os
from datetime import datetime


LOG_DIRNAME = 'log'
LOG_FILENAME_BASE = 'log'
LOG_FILENAME_EXTENSION = '.txt'


def get_formatter():
    """Get formatter to be used for generating log entries."""
    datefmt = '%H:%M:%S'

    level_str = '[%(levelname)s]'
    time_str = '[%(asctime)s]'
    location_str = '[%(filename)s:%(funcName)s:%(lineno)s]'
    message_str = ': %(message)s'
    format_string = level_str + time_str + location_str + message_str

    formatter = logging.Formatter(format_string, datefmt=datefmt)
    return formatter


def get_log_filename():
    """Get timestamped filename for a new log file."""
    format = '%y-%m-%d-%Hh-%Mm-%Ss'
    timestamp = datetime.now().strftime(format)
    log_filename = LOG_FILENAME_BASE + '_' + timestamp + LOG_FILENAME_EXTENSION
    fullname = os.path.join(LOG_DIRNAME, log_filename)
    return fullname


def str2loglevel(loglevel_str):
    """Convert string version of a log level to an enumeration from the logging module."""
    if loglevel_str == 'debug':
        return logging.DEBUG
    elif loglevel_str == 'info':
        return logging.INFO
    elif loglevel_str == 'warn':
        return logging.WARNING
    elif loglevel_str == 'error':
        return logging.ERROR
    else:
        raise ValueError(f'Unknown loglevel: {loglevel_str}')


def init_logging(loglevel_str='info'):
    """Initialize logging."""
    loglevel = str2loglevel(loglevel_str)
    Path(LOG_DIRNAME).mkdir(parents=True, exist_ok=True)
    formatter = get_formatter()
    stdout_handler = logging.StreamHandler(sys.stdout)
    filename = get_log_filename()
    file_handler = logging.FileHandler(filename)
    handlers = [stdout_handler, file_handler]

    for handler in handlers:
        handler.setFormatter(formatter)
        handler.setLevel(loglevel)

    logging.basicConfig(level=loglevel, handlers=handlers)