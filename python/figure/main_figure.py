import logging
from types import SimpleNamespace
import json

import matplotlib.pyplot as plt
import matplotlib as mpl
from matplotlib.lines import Line2D
import numpy as np

import python.utils.geometry as geometry
import python.utils.storage as storage
import python.utils.comparison_methods.gu2005 as gu2005
from python.figure.configurations import CONFIGURATIONS


EXAMPLE1_SCALING = 1.1
SINGLE_COLUMN_WIDTH = 8.85553
DOUBLE_COLUMN_WIDTH = 18.3436


logger = logging.getLogger(__name__)


def set_general_parameters():
    mpl.rcParams['font.family'] = 'serif'
    mpl.rcParams['font.serif'] = ['Computer Modern Roman']
    mpl.rcParams['text.usetex'] = True
    mpl.rcParams['axes.labelsize'] = 8
    mpl.rcParams['xtick.labelsize'] = 8
    mpl.rcParams['ytick.labelsize'] = 8
    mpl.rcParams['legend.fontsize'] = 7


def new_figure(width, height, tight=True, constrained_layout=False):
    INCH2CENT = 2.54
    size = width/INCH2CENT, height/INCH2CENT
    fig, ax = plt.subplots(figsize=size, constrained_layout=constrained_layout)
    if tight:
        fig.tight_layout(pad=0)
    return fig, ax


def read_data(args):
    data = None

    with open(f'output/data/{args.algorithm}/{args.system}.data', 'r') as read_file:
        data = json.load(read_file, object_hook=lambda d: SimpleNamespace(**d))

    if data == None:
        raise Exception(f'Error reading nu results from file')

    return data


def configure_ticks(ax, cfg):
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
    ax.xaxis.set_major_locator(mpl.ticker.MultipleLocator(cfg.ticks.major_x))
    ax.xaxis.set_minor_locator(mpl.ticker.MultipleLocator(cfg.ticks.minor_x))
    ax.yaxis.set_major_locator(mpl.ticker.MultipleLocator(cfg.ticks.major_y))
    ax.yaxis.set_minor_locator(mpl.ticker.MultipleLocator(cfg.ticks.minor_y))


def add_ray_to_ax(ax, ray, linecolor, linewidth):
    ray_start = ray.origin
    ray_end = geometry.theta2point(ray.origin, ray.length, ray.angle)
    line = np.vstack((ray_start, ray_end)).T
    ax.plot(line[0,:], line[1,:], zorder=1,
            c=linecolor, linewidth=linewidth, solid_capstyle='round')


def add_rayfan_to_ax(ax, rayfan, linecolor, linewidth, ratio, origins=False):
    ORIGIN_MARKERSTYLE = 'X'
    ORIGIN_MARKERCOLOR = 'black'
    ORIGIN_MARKERSIZE = 4

    for ray in rayfan.rays[0::ratio]:
        add_ray_to_ax(ax, ray, linecolor, linewidth)

    if origins:
        p1 = rayfan.origin[0]
        p2 = rayfan.origin[1]
        ax.plot(p1, p2, ORIGIN_MARKERSTYLE, color=ORIGIN_MARKERCOLOR,
                clip_on=False, markersize=ORIGIN_MARKERSIZE)




def create_figure(args):
    data = read_data(args)
    cfg = CONFIGURATIONS[args.system]

    set_general_parameters()

    # Fetch figure
    tight = False
    constrained = True
    width, height = SINGLE_COLUMN_WIDTH, SINGLE_COLUMN_WIDTH * EXAMPLE1_SCALING
    fig, ax = new_figure(width, height, tight, constrained)

    # Configure axes
    ax.set_xlim(data.limits.p1_min, data.limits.p1_max)
    ax.set_ylim(data.limits.p2_min, data.limits.p2_max)
    ax.set_xlabel(f'${data.parameters[0]}$')
    ax.set_ylabel(f'${data.parameters[1]}$')
    ax.xaxis.labelpad = 0
    ax.yaxis.labelpad = 0
    configure_ticks(ax, cfg)

    linewidth = 1
    origins = True
    colors = {0: 'g', 2: 'darkred', 4: 'cornflowerblue', 6: 'orange', 8: 'mediumpurple'}
    nus = set()

    for rayfan, ratio in zip(data.rayfans, cfg.ratios):
        nu = rayfan.nu
        nus.add(nu)
        color = colors[nu]
        logging.debug(f'Taking color {color} for nu {nu}')
        add_rayfan_to_ax(ax, rayfan, color, linewidth, ratio, origins)

    # Prepare regular legend labels
    nus = sorted(list(nus))
    legend_elements = [
        Line2D([0], [0], color=colors[nu], label=f'$NU_f$ = {nu}') for nu in nus]

    # Add origin label, if necessary
    if origins:
        origin_handle = Line2D([0], [0], color='black', linestyle='None',
            markersize=4, marker='X', label='Starting points'),
        legend_elements.insert(0, *origin_handle)

    # legend_elements = [
    #     # Line2D([0], [0], color='black', linestyle='--', label='SCS'),
    #     ]
    x = ax.legend(handles=legend_elements, loc='upper left', frameon=False,
              bbox_to_anchor=cfg.bbox, mode='expand', ncol=cfg.ncol)

    # Call custom drawing actions for given system
    cfg.custom_func(ax)

    return fig


def main(args):
    fig = create_figure(args)
    storage.save_figure(args, fig, args.command, args.algorithm)