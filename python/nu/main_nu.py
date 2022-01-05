import logging
import subprocess
import python.utils.rust_adapter as rust_utils


logger = logging.getLogger(__name__)


def main(args):
    logger.info(f'Invoking Rust subsystem for nu')

    rust_args = rust_utils.build_rust_command(args)
    result = subprocess.run(
        rust_args,
        cwd='./rust',
        stdout=None,
        stderr=None,
    )

    logger.info(f'Rust subsystem for nu complete')