import logging
from types import SimpleNamespace
import json

import matplotlib.pyplot as plt
import matplotlib as mpl
from matplotlib.lines import Line2D
import matplotlib.image as mpimg
import numpy as np
from PIL import Image, ImageDraw
from matplotlib.offsetbox import TextArea, DrawingArea, OffsetImage, AnnotationBbox

import python.utils.geometry as geometry
import python.utils.storage as storage
from python.figure.configurations import LINE_CONFIGURATIONS, REGION_CONFIGURATIONS
from python.figure.configurations import LineConfiguration, TickConfiguration
from python.figure.main_figure import set_general_parameters, new_figure_inches
from python.figure.main_figure import configure_ticks, add_rayfan_to_ax
from python.figure.main_figure import ORIGIN_LABEL_LINE


logger = logging.getLogger(__name__)


def read_data(path):
    data = None

    with open(path, 'r') as read_file:
        data = json.load(read_file, object_hook=lambda d: SimpleNamespace(**d))

    if data == None:
        raise Exception(f'Error reading nu results from file')

    return data


def instructional_line_sufficient(args):
    figure_cfg = LineConfiguration(
        width=3.486429134,
        height=3.486429134,
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
    )
    RAYFAN_INDEX = 0
    RAY_INDEX = 90
    LABEL_OFFSET = (-3,-8)

    data = read_data(f'output/data/line/pde_complex_tau_sigma_instructional.data')

    set_general_parameters()

    # Fetch figure
    tight = False
    constrained = True
    width, height = figure_cfg.width, figure_cfg.height
    fig, ax = new_figure_inches(width, height, tight, constrained)

    # Configure axes
    ax.set_xlim(data.limits.p1_min, data.limits.p1_max)
    ax.set_ylim(data.limits.p2_min, data.limits.p2_max)
    ax.set_xlabel(f'${data.parameters[0]}$')
    ax.set_ylabel(f'${data.parameters[1]}$')
    ax.xaxis.labelpad = 0
    ax.yaxis.labelpad = 0
    configure_ticks(ax, figure_cfg)

    linewidth = 1

    if figure_cfg.ratios == None:
        ratios = [1] * len(data.rayfans)
    else:
        ratios = figure_cfg.ratios

    # Select instructional rayfan
    rayfan = data.rayfans[RAYFAN_INDEX]
    ray = rayfan.rays[RAY_INDEX]
    
    # Add origin
    ax.plot(rayfan.origin[0], rayfan.origin[1], 'o', color='black', markersize=3)
    ax.annotate(r'$\eta^0$', rayfan.origin, textcoords='offset points',
                xytext=LABEL_OFFSET , ha='right')
    
    # Calculate intermediate points
    intermediate = [geometry.theta2point(ray.origin, s, ray.angle) for s in ray.segments]

    # Add dashed line
    ray_start = ray.origin
    ray_end = geometry.theta2point(ray.origin, ray.length, ray.angle)
    line = np.vstack((ray_start, ray_end)).T
    ax.plot(
        line[0,:],
        line[1,:],
        '--',
        color='black',
        linewidth=linewidth,
        solid_capstyle='round'
    )

    # Add all intermediate points to plot
    for point in intermediate:
        ax.plot(point[0], point[1], 'o', color='black', markersize=3)

    # Add \eta^1
    ax.plot(intermediate[0][0], intermediate[0][1], 'o', color='black', markersize=3)
    ax.annotate(
        fr'$\eta^1$',
        intermediate[0],
        textcoords='offset points',
        xytext=LABEL_OFFSET,
        ha='right'
    )

    # Add final point
    ax.plot(intermediate[-1][0], intermediate[-1][1], 'o', color='black', markersize=3)
    ax.annotate(
        fr'$\eta^{{{len(intermediate)}}}$',
        intermediate[-1],
        textcoords='offset points',
        xytext=LABEL_OFFSET,
        ha='right'
    )

    # Prepare regular legend labels
    # nus = sorted(list(nus))
    # legend_handles = [
    #     Line2D([0], [0], color=colors[nu], label=f'$NU_f$ = {nu}') for nu in nus]

    # Draw the legend
    # ax.legend(handles=legend_handles, loc='upper left', frameon=False,
    #           bbox_to_anchor=figure_cfg.bbox, mode='expand', ncol=figure_cfg.ncol)

    path = 'test.pdf'
    fig.savefig(path, dpi=1000)


def main(args):
    logger.info(f'Running custom script {args.customscript}!')

    if args.customscript == 'instructional_line_sufficient':
        instructional_line_sufficient(args)