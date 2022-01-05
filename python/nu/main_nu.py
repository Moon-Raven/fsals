import logging
import subprocess
import python.utils.rust_adapter as rust_utils
import contextlib


logger = logging.getLogger(__name__)


def calculate_nu(args):
    rust_args = rust_utils.build_rust_command(args)
    logger.info(f'Invoking Rust subsystem for nu')

    result = None
    with contextlib.redirect_stdout(logging.getLogger(__name__)):
        result = subprocess.run(
            rust_args,
            cwd='./rust',
            stdout=None,
            stderr=None,
        )

    if result.returncode != 0:
        raise Exception(f'Rust subsystem for nu exited with {result.returncode}')

    logger.info(f'Rust subsystem for nu complete')


def create_figure(args):
    pass


def save_figure(args, fig):
    pass


def main(args):
    calculate_nu(args)
    fig = create_figure(args)
    save_figure(args, fig)