import logging


logger = logging.getLogger(__name__)


def build_rust_command(args):
    rust_command = ['cargo', 'run', '--']
    if args.algorithm is not None:
        rust_command += ['-a', args.algorithm]
    if args.system is not None:
        rust_command += ['-s', args.system]
    if args.parallel:
        rust_command += ['--parallel']
    rust_command += ['--loglevel', args.loglevel]
    rust_command += [args.command]
    return rust_command