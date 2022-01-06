from datetime import datetime

def get_timestamp_str():
    now = datetime.now()
    timestamp = now.strftime('%d-%b-%Y_%H:%M:%S')
    return timestamp