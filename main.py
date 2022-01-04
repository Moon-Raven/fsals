import argparse

import python.custom.main_custom
import python.data.main_data
import python.figure.main_figure
import python.nu.main_nu


def parse_cli_arguments():
    arg_parser = argparse.ArgumentParser()

    arg_parser.add_argument(
        'Command',
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

    args = arg_parser.parse_args()
    return args


def main():
    args = parse_cli_arguments()

    if args.Command == 'custom':
        python.custom.main_custom.main(args)
    elif args.Command == 'data':
        python.data.main_data.main(args)
    elif args.Command == 'figure':
        python.figure.main_figure.main(args)
    elif args.Command == 'nu':
        python.nu.main_nu.main(args)
    else:
        raise argparse.ArgumentError(f'Unknown command: {args.Command}')


if __name__ == '__main__':
    main()