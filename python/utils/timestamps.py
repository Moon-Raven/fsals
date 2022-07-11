"""This module contains facilities for generating formatted timestamps."""
from datetime import datetime

def get_timestamp_str():
    """Generate a formatted timestamp of current datetime."""
    now = datetime.now()
    timestamp = now.strftime('%d-%b-%Y_%H-%M-%S')
    return timestamp