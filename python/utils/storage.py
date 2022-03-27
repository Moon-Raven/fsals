from pathlib import Path
from shutil import copyfile

import python.utils.timestamps as timestamps


def save_figure(args, fig, command, subcommand, extension):
    dirname = f'output/{command}/{subcommand}'
    dir = Path(dirname)
    dir.mkdir(exist_ok=True, parents=True)
    timestamp = timestamps.get_timestamp_str()
    figpath_timestamped = f'{dirname}/{args.system}_{timestamp}_{subcommand}.{extension}'
    figpath = f'{dirname}/{args.system}_{subcommand}.{extension}'
    fig.savefig(figpath_timestamped, dpi=1000)
    copyfile(figpath_timestamped, figpath)