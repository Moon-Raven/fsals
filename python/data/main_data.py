import logging
import subprocess
import contextlib

import python.utils.rust_adapter as rust_utils


logger = logging.getLogger(__name__)


def main(args):
    rust_args = rust_utils.build_rust_command(args)
    logger.info(f'Invoking Rust subsystem for data generation')

    result = None
    with contextlib.redirect_stdout(logging.getLogger(__name__)):
        result = subprocess.run(rust_args, cwd='./rust', stdout=None, stderr=None)

    if result.returncode != 0:
        raise Exception(f'Rust subsystem for data gen exited with {result.returncode}')

    logger.info(f'Rust subsystem for data generation complete')