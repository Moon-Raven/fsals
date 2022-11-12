"""This module contains facilities for reading/saving data/figures to/from files."""
import logging
from pathlib import Path
from shutil import copyfile
from types import SimpleNamespace
from typing import Any
import json
from json.encoder import JSONEncoder

import python.utils.timestamps as timestamps


logger = logging.getLogger(__name__)


class SimpleNamespaceJSONEncoder(JSONEncoder):
    def default(self, o: Any) -> Any:
        if isinstance(o, SimpleNamespace):
            return o.__dict__
        return super().default(o)


def read_data(args, conf):
    """Read fsals results from storage."""
    data = None

    path = f'output/data/{args.algorithm}/{conf.rust_configuration}.data'
    with open(path, 'r') as read_file:
        logging.info(f'Reading data from {path}')
        data = json.load(read_file, object_hook=lambda d: SimpleNamespace(**d))

    if data == None:
        raise Exception(f'Error reading nu results from file')

    return data


def read_data_from_path(path):
    """Read fsals results directly from specified file."""
    data = None

    with open(path, 'r') as read_file:
        data = json.load(read_file, object_hook=lambda d: SimpleNamespace(**d))

    if data == None:
        raise Exception(f'Error reading nu results from file')

    return data


def write_data_to_path(data, path):
    """
        Save fsals results directly to specified path.
        Provided as a supplementary function.
    """
    with open(path, 'w') as write_file:
        logging.info(f'Writing data to {path}')
        json.dump(data, write_file, cls=SimpleNamespaceJSONEncoder)


def save_figure(args, fig, command, subcommand, extension):
    """Save given figure to the filesystem."""
    dirname = f'output/{command}/{subcommand}'
    dir = Path(dirname)
    dir.mkdir(exist_ok=True, parents=True)
    timestamp = timestamps.get_timestamp_str()
    figpath_timestamped = f'{dirname}/{args.configuration}_{timestamp}_{subcommand}.{extension}'
    figpath = f'{dirname}/{args.configuration}_{subcommand}.{extension}'
    fig.savefig(figpath_timestamped, dpi=1000)
    copyfile(figpath_timestamped, figpath)