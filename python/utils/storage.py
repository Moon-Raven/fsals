from pathlib import Path
from shutil import copyfile

import python.utils.timestamps as timestamps


def save_figure(args, fig, command, subcommand):
    dirname = f'output/{command}/{subcommand}'
    dir = Path(dirname)
    dir.mkdir(exist_ok=True, parents=True)
    timestamp = timestamps.get_timestamp_str()
    figpath_timestamped = f'{dirname}/{args.system}_{timestamp}.pdf'
    figpath = f'{dirname}/{args.system}.pdf'

    fig.savefig(figpath_timestamped)
    copyfile(figpath_timestamped, figpath)