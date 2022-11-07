"""This module contains facilities for invoking the rust subystem."""
import logging


logger = logging.getLogger(__name__)


def build_rust_command(args):
    """Build a rust command to be run via cargo."""
    rust_command = ['cargo', 'run', '--release', '--']
    if args.algorithm is not None:
        rust_command += ['-a', args.algorithm]
    if args.configuration is not None:
        rust_command += ['-c', args.configuration]
    if args.parallel:
        rust_command += ['--parallel']
    rust_command += ['--loglevel', args.loglevel]
    rust_command += [args.command]
    return rust_command