import logging
import subprocess
import contextlib
from types import SimpleNamespace
import json
from pathlib import Path

import matplotlib as mpl
import matplotlib.pyplot as plt

import python.utils.rust_adapter as rust_utils


SINGLE_COLUMN_WIDTH = 8.43
DOUBLE_COLUMN_WIDTH = 17.5
EXAMPLE1_SCALING = 1.1


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


def new_figure(width, height, tight=True, constrained_layout=False):
    INCH2CENT = 2.54
    size = width/INCH2CENT, height/INCH2CENT
    fig, ax = plt.subplots(figsize=size, constrained_layout=constrained_layout)
    if tight:
        fig.tight_layout(pad=0)
    return fig, ax


def set_general_parameters():
    mpl.rcParams['font.family'] = 'serif'
    mpl.rcParams['font.serif'] = ['Computer Modern Roman']
    mpl.rcParams['text.usetex'] = True
    mpl.rcParams['axes.labelsize'] = 8
    mpl.rcParams['xtick.labelsize'] = 8
    mpl.rcParams['ytick.labelsize'] = 8
    mpl.rcParams['legend.fontsize'] = 7


def configure_ticks(ax):
    MAJOR_SIZE = 3
    MINOR_SIZE = 2
    MAJOR_WIDTH = 0.9
    MINOR_WIDTH = 0.7
    ax.xaxis.set_tick_params(which='major', size=MAJOR_SIZE,
                             width=MAJOR_WIDTH, direction='in', top='on')
    ax.xaxis.set_tick_params(which='minor', size=MINOR_SIZE,
                             width=MINOR_WIDTH, direction='in', top='on')
    ax.yaxis.set_tick_params(which='major', size=MAJOR_SIZE,
                             width=MAJOR_WIDTH, direction='in', right='on')
    ax.yaxis.set_tick_params(which='minor', size=MINOR_SIZE,
                             width=MINOR_WIDTH, direction='in', right='on')


def nu2color(nu):
    MIN_NU_THRESHOLD = 1
    MAX_NU_THRESHOLD = 10
    MIN_NU_SATURATION = 0.6
    MAX_NU_SATURATION = 1
    saturation_range = MAX_NU_SATURATION - MIN_NU_SATURATION
    nu_cut = min(nu, MAX_NU_THRESHOLD)
    ratio = (nu_cut - MIN_NU_THRESHOLD)/(MAX_NU_THRESHOLD - MIN_NU_THRESHOLD)
    saturation = MIN_NU_SATURATION + ratio * saturation_range
    red = int(128 + saturation * (255-128))
    other = int(128 * (1-saturation))
    color = (red/255, other/255, other/255)
    return color


def create_figure(args):
    nu_results = None

    with open(f'output/nu/temp_data/{args.system}.nudata', 'r') as read_file:
        nu_results = json.load(read_file, object_hook=lambda d: SimpleNamespace(**d))

    if nu_results == None:
        raise Exception(f'Error reading nu results from file')

    # set_general_parameters()

    # Fetch figure
    # tight = False
    # constrained = True
    # width, height = SINGLE_COLUMN_WIDTH, SINGLE_COLUMN_WIDTH*EXAMPLE1_SCALING
    # fig, ax = new_figure(width, height, tight, constrained)
    fig, ax = plt.subplots()

    # Configure axes
    ax.set_xlim(nu_results.limits.p1_min, nu_results.limits.p1_max)
    ax.set_ylim(nu_results.limits.p2_min, nu_results.limits.p2_max)
    ax.set_xlabel(f'${nu_results.parameters[0]}$')
    ax.set_ylabel(f'${nu_results.parameters[1]}$')
    # ax.xaxis.labelpad = -3
    # ax.yaxis.labelpad = 0
    # configure_ticks(ax)
    # ax.xaxis.set_major_locator(mpl.ticker.MultipleLocator(1))
    # ax.xaxis.set_minor_locator(mpl.ticker.MultipleLocator(0.25))
    # ax.yaxis.set_major_locator(mpl.ticker.MultipleLocator(1))
    # ax.yaxis.set_minor_locator(mpl.ticker.MultipleLocator(0.25))

    for point in nu_results.point_results:
        if point.nu > 0:
            color = nu2color(point.nu)
        else:
            color = 'g'
        ax.text(point.p[0], point.p[1], str(int(point.nu)), color=color)
        ax.plot(point.p[0], point.p[1], 'bo', markersize=1)

    return fig


def save_figure(args, fig):
    dirname = 'output/nu/figures'
    dir = Path(dirname)
    dir.mkdir(exist_ok=True, parents=True)
    figpath = f'{dirname}/{args.system}.pdf'
    fig.savefig(figpath)


def main(args):
    calculate_nu(args)
    fig = create_figure(args)
    save_figure(args, fig)