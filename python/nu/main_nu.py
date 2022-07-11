import logging
import subprocess
import contextlib
from types import SimpleNamespace
import json

import matplotlib.pyplot as plt

import python.utils.rust_adapter as rust_utils
import python.utils.storage as storage


logger = logging.getLogger(__name__)


def calculate_nu(args):
    """ Invoke rust subsystem to calculate nu for given configuration. """

    rust_args = rust_utils.build_rust_command(args)
    logger.info(f'Invoking Rust subsystem for nu')

    result = None
    with contextlib.redirect_stdout(logging.getLogger(__name__)):
        result = subprocess.run(rust_args, cwd='./rust', stdout=None, stderr=None)

    if result.returncode != 0:
        raise Exception(f'Rust subsystem for nu exited with {result.returncode}')

    logger.info(f'Rust subsystem for nu complete')


def nu2color(nu):
    """ Determine the color of the number for this nu. """

    COLOR_MAX = 255
    COLOR_MID = 128

    MIN_NU_THRESHOLD = 1     # At which nu do labels become red
    MAX_NU_THRESHOLD = 10    # At which nu do labels become reddest
    MIN_NU_SATURATION = 0.6  # How red are the least red labels
    MAX_NU_SATURATION = 1    # How red are the most red labels
    saturation_range = MAX_NU_SATURATION - MIN_NU_SATURATION # Range od redness

    # Calculate color of given nu (a bit of overengineering which is unimportant)
    nu_cut = min(nu, MAX_NU_THRESHOLD) # Compare nu to threshold
    ratio = (nu_cut - MIN_NU_THRESHOLD)/(MAX_NU_THRESHOLD - MIN_NU_THRESHOLD)
    saturation = MIN_NU_SATURATION + ratio * saturation_range
    red = int(COLOR_MID + saturation * (COLOR_MAX-COLOR_MID))
    blue_and_green = int(COLOR_MID * (1-saturation))
    color = (red/COLOR_MAX, blue_and_green/COLOR_MAX, blue_and_green/COLOR_MAX)

    return color


def create_figure(args):
    """ Visualize nu data on a new figure. """

    nu_results = None

    with open(f'output/nu/temp_data/{args.configuration}.nudata', 'r') as read_file:
        nu_results = json.load(read_file, object_hook=lambda d: SimpleNamespace(**d))

    if nu_results == None:
        raise Exception(f'Error reading nu results from file')

    fig, ax = plt.subplots()

    # Configure axes
    ax.set_xlim(nu_results.limits.p1_min, nu_results.limits.p1_max)
    ax.set_ylim(nu_results.limits.p2_min, nu_results.limits.p2_max)
    ax.set_xlabel(f'${nu_results.parameters[0]}$')
    ax.set_ylabel(f'${nu_results.parameters[1]}$')

    # Add points
    for point in nu_results.point_results:
        if point.nu > 0:
            color = nu2color(point.nu)
        else:
            color = 'g'
        ax.text(point.p[0], point.p[1], str(int(point.nu)), color=color)
        ax.plot(point.p[0], point.p[1], 'bo', markersize=1)

    return fig


def main(args):
    """ Calculate, visualize and store nu for given configuration. """
    calculate_nu(args)
    fig = create_figure(args)
    extension = 'pdf'
    storage.save_figure(args, fig, 'nu', 'figure', extension)