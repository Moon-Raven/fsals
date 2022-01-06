import argparse
import logging
import time

import python.custom.main_custom
import python.data.main_data
import python.figure.main_figure
import python.nu.main_nu
import python.utils.log_helper


logger = None


def log_args(args):
    logger.info('Running script with following parameters:')
    logger.info(f'  Command: {args.command}')
    logger.info(f'  Algorithm: {args.algorithm}')
    logger.info(f'  System: {args.system}')
    logger.info(f'  Parallel: {args.parallel}')
    logger.info(f'  LogLevel: {args.loglevel}')


def parse_cli_arguments():
    arg_parser = argparse.ArgumentParser()

    arg_parser.add_argument(
        'command',
        metavar='command',
        type=str,
        help='command to run',
        choices=['data', 'figure', 'nu', 'custom'])

    arg_parser.add_argument('-s', '--system', help='system to analyze')

    arg_parser.add_argument(
        '-a',
        '--algorithm',
        help='algorithm to run',
        choices=['line', 'region'])

    arg_parser.add_argument(
        '-p',
        '--parallel',
        action='store_true',
        help='parellelize code execution')

    arg_parser.add_argument(
        '-l',
        '--loglevel',
        help='logging level',
        choices=['debug', 'info', 'warn', 'error'],
        default='info')

    args = arg_parser.parse_args()
    return args


def main():
    global logger

    start = time.monotonic()
    args = parse_cli_arguments()
    python.utils.log_helper.init_logging(args.loglevel)
    logger = logging.getLogger(__name__)
    log_args(args)

    if args.command == 'custom':
        python.custom.main_custom.main(args)
    elif args.command == 'data':
        python.data.main_data.main(args)
    elif args.command == 'figure':
        python.figure.main_figure.main(args)
    elif args.command == 'nu':
        python.nu.main_nu.main(args)
    else:
        raise argparse.ArgumentError(f'Unknown command: {args.command}')

    end = time.monotonic()
    elapsed = end - start
    logger.info(f'Python script completed in {elapsed} seconds')


if __name__ == '__main__':
    main()