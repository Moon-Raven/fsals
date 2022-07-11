import logging
import subprocess
from pathlib import Path
from shutil import copyfile

import python.utils.rust_adapter as rust_utils
import python.utils.timestamps as timestamps

logger = logging.getLogger(__name__)


def main(args):
    """ Invoke rust subsystem to run fsals for given configuration. """

    rust_args = rust_utils.build_rust_command(args)
    logger.info(f'Invoking Rust subsystem for data generation')

    dirname = f'log/rust'
    dir = Path(dirname)
    dir.mkdir(exist_ok=True, parents=True)
    timestamp = timestamps.get_timestamp_str()
    rust_log_filename_timestamped = f'{dirname}/{timestamp}.txt'
    rust_log_filename_last = f'{dirname}/last_log.txt'
    f = open(rust_log_filename_timestamped, 'w')

    result = None
    result = subprocess.run(rust_args, cwd='./rust', stdout=f, stderr=f)
    copyfile(rust_log_filename_timestamped, rust_log_filename_last)

    if result.returncode != 0:
        raise Exception(f'Rust subsystem for data gen exited with {result.returncode}')

    logger.info(f'Rust subsystem for data generation complete')