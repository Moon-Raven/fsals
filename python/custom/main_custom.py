import logging
from types import SimpleNamespace
import json
from pathlib import Path

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


def instructional_line_nsc(args):
    figure_cfg = LineConfiguration(
        width=3.486429134,
        height=3.486429134 * 1.05,
        ncol=3,
        bbox=(0, -0.21, 1, 0.1),
    )
    
    # Configuration
    RAYFAN_INDEX = 0
    RAY_INDEX = 90
    LABEL_OFFSET = (-3,-8)
    LINEWIDTH_THICK = 1
    LINEWIDTH_THIN = 0.5
    set_general_parameters()

    # Fetch figure
    tight = False
    constrained = True
    width, height = figure_cfg.width, figure_cfg.height
    fig, ax = new_figure_inches(width, height, tight, constrained)

    # Configure axes
    data = read_data(f'output/data/line/pde_complex_tau_sigma_instructional.data')
    ax.set_xlim(data.limits.p1_min, data.limits.p1_max)
    ax.set_ylim(data.limits.p2_min, data.limits.p2_max)
    ax.set_xlabel(f'${data.parameters[0]}$')
    ax.set_ylabel(f'${data.parameters[1]}$')
    ax.xaxis.labelpad = 0
    ax.yaxis.labelpad = 0
    configure_ticks(ax, figure_cfg)

    # Fetch instructional ray
    rayfan = data.rayfans[RAYFAN_INDEX]
    ray = rayfan.rays[RAY_INDEX]
    
    # Add origin
    ax.plot(rayfan.origin[0], rayfan.origin[1], 'o', color='black', markersize=3)
    ax.annotate(r'$\eta^0$', rayfan.origin, textcoords='offset points',
                xytext=LABEL_OFFSET , ha='right')
    
    # Calculate intermediate points
    intermediate = [geometry.theta2point(ray.origin, s, ray.angle) for s in ray.segments]

    # Add dashed line resembling the ray
    ray_start = np.array(ray.origin)
    beta = np.pi/2 - np.abs(ray.angle)
    tau_start, sigma_start = ray_start[0], ray_start[1]
    tau_end = tau_start + sigma_start * np.tan(beta)
    sigma_end = 0
    line = np.vstack((ray_start, [tau_end, sigma_end])).T
    ax.plot(
        line[0,:],
        line[1,:],
        '--',
        color='black',
        linewidth=LINEWIDTH_THIN,
        solid_capstyle='round',
        label=r'Kriva $\eta(\theta)$',
    )

    # Add dashed line
    ray_start = ray.origin
    ray_end = geometry.theta2point(ray.origin, ray.length, ray.angle)
    line = np.vstack((ray_start, ray_end)).T
    ax.plot(
        line[0,:],
        line[1,:],
        color='black',
        linewidth=LINEWIDTH_THICK,
        solid_capstyle='round',
        label='Segment ekvivalentne stabilnosti',
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

    # Draw the legend
    ax.legend(loc='lower center', frameon=False, bbox_to_anchor=figure_cfg.bbox)

    # Save fig
    dirname = f'output/custom'
    dir = Path(dirname)
    dir.mkdir(exist_ok=True, parents=True)
    figpath = f'{dirname}/{args.customscript}.pdf'
    fig.savefig(figpath, dpi=1000)


def instructional_line_nsc_multiple(args):
    figure_cfg = LineConfiguration(
        width=3.486429134,
        height=3.486429134 * 1.05,
        ncol=3,
        bbox=(0, -0.21, 1, 0.1),
    )
    
    # Configuration
    RAYFAN_INDEX = 0
    RAY_INDEX = 90
    LABEL_OFFSET = (-3,-8)
    LINEWIDTH_THICK = 1
    LINEWIDTH_THIN = 0.5
    set_general_parameters()

    # Fetch figure
    tight = False
    constrained = True
    width, height = figure_cfg.width, figure_cfg.height
    fig, ax = new_figure_inches(width, height, tight, constrained)

    # Configure axes
    data = read_data(f'output/data/line/pde_complex_tau_sigma_instructional.data')
    ax.set_xlim(data.limits.p1_min, data.limits.p1_max)
    ax.set_ylim(data.limits.p2_min, data.limits.p2_max)
    ax.set_xlabel(f'${data.parameters[0]}$')
    ax.set_ylabel(f'${data.parameters[1]}$')
    ax.xaxis.labelpad = 0
    ax.yaxis.labelpad = 0
    configure_ticks(ax, figure_cfg)

    # Fetch instructional ray
    rayfan = data.rayfans[RAYFAN_INDEX]
    for ray in rayfan.rays[::8]:
        # Add origin
        ax.plot(rayfan.origin[0], rayfan.origin[1], 'o', color='black', markersize=3)
        
        # Calculate intermediate points
        intermediate = [geometry.theta2point(ray.origin, s, ray.angle) for s in ray.segments]

        # Add dashed lines resembling the ray
        ray_start = np.array(ray.origin)
        beta = np.pi/2 - ray.angle
        tau_start, sigma_start = ray_start[0], ray_start[1]
        tau_end = tau_start + sigma_start * np.tan(beta)
        if ray.angle > 0 and ray.angle < np.pi:
            sigma_end = 20.0
        else:
            sigma_end = 0
        line = np.vstack((ray_start, [tau_end, sigma_end])).T
        ax.plot(
            line[0,:],
            line[1,:],
            '--',
            color='black',
            linewidth=LINEWIDTH_THIN,
            solid_capstyle='round',
            label=r'Kriva $\eta(\theta)$',
        )

        # Add dashed line
        ray_start = ray.origin
        ray_end = geometry.theta2point(ray.origin, ray.length, ray.angle)
        line = np.vstack((ray_start, ray_end)).T
        ax.plot(
            line[0,:],
            line[1,:],
            color='black',
            linewidth=LINEWIDTH_THICK,
            solid_capstyle='round',
            label='Segment ekvivalentne stabilnosti',
        )

        # Add all intermediate points to plot
        for point in intermediate:
            ax.plot(point[0], point[1], 'o', color='black', markersize=3)

        # Add \eta^1
        ax.plot(intermediate[0][0], intermediate[0][1], 'o', color='black', markersize=3)

        # Add final point
        ax.plot(intermediate[-1][0], intermediate[-1][1], 'o', color='black', markersize=3)

    # Draw the legend
    legend_handles = [
        Line2D(
            [0], [0],
            linestyle='--',
            linewidth= LINEWIDTH_THIN,
            color='black',
            label=r'Krive $\eta(\theta)$'),
        Line2D(
            [0], [0],
            linestyle='-',
            linewidth= LINEWIDTH_THICK,
            color='black',
            label='Segmenti ekvivalente stabilnosti')
    ]
    ax.legend(
        handles=legend_handles,
        loc='lower center',
        frameon=False,
        bbox_to_anchor=figure_cfg.bbox
    )

    # Save fig
    dirname = f'output/custom'
    dir = Path(dirname)
    dir.mkdir(exist_ok=True, parents=True)
    figpath = f'{dirname}/{args.customscript}.pdf'
    fig.savefig(figpath, dpi=1000)


def instructional_line_sufficient(args):
    figure_cfg = LineConfiguration(
        width=3.486429134,
        height=3.486429134 * 1.05,
        ncol=3,
        bbox=(0, -0.21, 1, 0.1),
    )
    
    # Configuration
    RAYFAN_INDEX = 0
    RAY_INDEX = 90
    LABEL_OFFSET = (-3,-8)
    LINEWIDTH_THICK = 1
    LINEWIDTH_THIN = 0.5
    set_general_parameters()

    # Fetch figure
    tight = False
    constrained = True
    width, height = figure_cfg.width, figure_cfg.height
    fig, ax = new_figure_inches(width, height, tight, constrained)

    # Configure axes
    data = read_data(f'output/data/line/pde_complex_tau_sigma_instructional.data')
    ax.set_xlim(data.limits.p1_min, data.limits.p1_max)
    ax.set_ylim(data.limits.p2_min, data.limits.p2_max)
    ax.set_xlabel(f'${data.parameters[0]}$')
    ax.set_ylabel(f'${data.parameters[1]}$')
    ax.xaxis.labelpad = 0
    ax.yaxis.labelpad = 0
    configure_ticks(ax, figure_cfg)

    # Fetch instructional ray
    rayfan = data.rayfans[RAYFAN_INDEX]
    ray = rayfan.rays[RAY_INDEX]
    
    # Add origin
    ax.plot(rayfan.origin[0], rayfan.origin[1], 'o', color='black', markersize=3)
    ax.annotate(r'$\eta^0$', rayfan.origin, textcoords='offset points',
                xytext=LABEL_OFFSET , ha='right')
    
    # Calculate intermediate points
    intermediate = [geometry.theta2point(ray.origin, s, ray.angle) for s in ray.segments]

    # Add line between \eta^0 and \eta^1
    ray_start = ray.origin
    ray_end = geometry.theta2point(ray.origin, ray.segments[0], ray.angle)
    line = np.vstack((ray_start, ray_end)).T
    ax.plot(
        line[0,:],
        line[1,:],
        color='black',
        linewidth=LINEWIDTH_THICK,
        solid_capstyle='round',
        label='Segment ekvivalentne stabilnosti',
    )

    # Add dashed line resembling the ray
    ray_start = np.array(ray.origin)
    beta = np.pi/2 - np.abs(ray.angle)
    tau_start, sigma_start = ray_start[0], ray_start[1]
    tau_end = tau_start + sigma_start * np.tan(beta)
    sigma_end = 0
    line = np.vstack((ray_start, [tau_end, sigma_end])).T
    ax.plot(
        line[0,:],
        line[1,:],
        '--',
        color='black',
        linewidth=LINEWIDTH_THIN,
        solid_capstyle='round',
        label=r'Kriva $\eta(\theta)$',
    )

    # Add all intermediate points to plot
    eta_1 = intermediate[0]
    ax.plot(eta_1[0], eta_1[1], 'o', color='black', markersize=3)

    # Add \eta^1
    ax.plot(intermediate[0][0], intermediate[0][1], 'o', color='black', markersize=3)
    ax.annotate(
        fr'$\eta^1$',
        intermediate[0],
        textcoords='offset points',
        xytext=LABEL_OFFSET,
        ha='right'
    )

    # Draw the legend
    ax.legend(loc='lower center', frameon=False, bbox_to_anchor=figure_cfg.bbox)

    # Save fig
    dirname = f'output/custom'
    dir = Path(dirname)
    dir.mkdir(exist_ok=True, parents=True)
    figpath = f'{dirname}/{args.customscript}.pdf'
    fig.savefig(figpath, dpi=1000)


def main(args):
    logger.info(f'Running custom script {args.customscript}!')

    if args.customscript == 'instructional_line_nsc':
        instructional_line_nsc(args)
    elif args.customscript == 'instructional_line_sufficient':
        instructional_line_sufficient(args)
    elif args.customscript == 'instructional_line_nsc_multiple':
        instructional_line_nsc_multiple(args)
    else:
        raise Exception(f'Unknown custom script {args.customscript}')