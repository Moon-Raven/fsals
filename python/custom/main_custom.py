"""Module for functions performing non-general and non-standard fsals actions."""
import logging
from types import SimpleNamespace
import json
from pathlib import Path

import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
from matplotlib.lines import Line2D
import matplotlib.ticker as ticker
import numpy as np

import python.utils.geometry as geometry
from python.figure.configurations import LineConfiguration
from python.figure.configurations import RegionConfiguration, TickConfiguration
from python.figure.main_figure import set_general_parameters, new_figure_inches
from python.figure.main_figure import configure_ticks
from python.figure.main_figure import get_ax_ratio, get_corners
from python.figure.main_figure import get_image_dimensions, get_drawable_canvas
from python.figure.main_figure import corners2pixels
import python.utils.storage as storage


THESIS_FIGWIDTH = 3.256429134

logger = logging.getLogger(__name__)


def instructional_line_nsc(args):
    """Draw the complete line fsals algorithm."""

    # Create a line configuration as a container for figure parameters
    figure_cfg = LineConfiguration(
        width=THESIS_FIGWIDTH,
        height=THESIS_FIGWIDTH * 1.05,
        ncol=3,
        bbox=(0, -0.21, 1, 0.1),
        ticks=TickConfiguration(5, 5, 1, 1),
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
    data = storage.read_data_from_path(f'output/data/line/pde_complex_tau_sigma_instructional.data')
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

    # Add \eta^1 and \eta^2
    for i in range(0, 3):
        ax.plot(intermediate[i][0], intermediate[i][1], 'o', color='black', markersize=3)
        ax.annotate(
            fr'$\eta^{i+1}$',
            intermediate[i],
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
    """Draw several steps of the fsals line algorithm."""
    figure_cfg = LineConfiguration(
        width=THESIS_FIGWIDTH,
        height=THESIS_FIGWIDTH * 1.05,
        ncol=3,
        bbox=(0, -0.21, 1, 0.1),
        ticks=TickConfiguration(5, 5, 1, 1),
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
    data = storage.read_data_from_path(f'output/data/line/pde_complex_tau_sigma_instructional.data')
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
    """Draw a single sufficient step of the fsals line algorithm."""
    figure_cfg = LineConfiguration(
        width=THESIS_FIGWIDTH,
        height=THESIS_FIGWIDTH * 1.05,
        ncol=3,
        bbox=(0, -0.21, 1, 0.1),
        ticks=TickConfiguration(5, 5, 1, 1),
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
    data = storage.read_data_from_path(f'output/data/line/pde_complex_tau_sigma_instructional.data')
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
    ax.plot(
        intermediate[0][0],
        intermediate[0][1],
        'o',
        color='black',
        markersize=3,
        )
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


def add_pregions_to_ax(fig, ax, pregions, limits, color, last_color=None, deep_origins=False):
    """Add pregions go given axes object."""
    max_depth = max([p.depth for p in pregions])
    ratio = get_ax_ratio(fig, ax)
    width, height = get_image_dimensions(ratio)
    pixel_dimensions = np.array([width, height])

    image, canvas = get_drawable_canvas(width, height)
    p1span = limits.p1_max - limits.p1_min
    p2span = limits.p2_max - limits.p2_min
    spans = np.array([p1span, p2span])
    mins = np.array([limits.p1_min, limits.p2_min])

    # Add deepest pregions
    red_pregions = [pregion for pregion in pregions if pregion.depth == max_depth]
    blue_pregions = [pregion for pregion in pregions if pregion.depth != max_depth]

    for pregion in red_pregions:
        corners = get_corners(pregion)
        upper_np, lower_np = corners2pixels(corners, spans, pixel_dimensions, mins)
        upper, lower = (upper_np[0], upper_np[1]), (lower_np[0], lower_np[1])
        canvas.ellipse([upper, lower], fill=last_color)

    # Add remaining pregions
    for pregion in blue_pregions:
        corners = get_corners(pregion)
        upper_np, lower_np = corners2pixels(corners, spans, pixel_dimensions, mins)
        upper, lower = (upper_np[0], upper_np[1]), (lower_np[0], lower_np[1])
        canvas.ellipse([upper, lower], fill=color)

    # Add origins
    if deep_origins:
        for pregion in red_pregions:
            x, y = pregion.origin[1], pregion.origin[1]
            ax.plot(
                x,
                y,
                linestyle='',
                marker=',',
                color='black',
                markersize=0.01,
                fillstyle='full',
            )

    box = [limits.p1_min, limits.p1_max, limits.p2_min, limits.p2_max]
    ax.imshow(image, extent=box, aspect='auto', origin='lower')


def instructional_region_sufficient(args):
    """Draw a single sufficient fsals region step."""
    data = storage.read_data_from_path(f'output/data/region/pde_complex_instructional.data')
    cfg = RegionConfiguration(
        width=THESIS_FIGWIDTH,
        height=THESIS_FIGWIDTH * 1.06,
        ncol=2,
        bbox=(0, -0.19, 1, 0.1),
        ticks=TickConfiguration(5, 5, 1, 1),
    )
    PREGION_COLOR = 'lightsteelblue'

    set_general_parameters()

    # Fetch figure
    tight = False
    constrained = True
    width, height = cfg.width, cfg.height
    fig, ax = new_figure_inches(width, height, tight, constrained)

    # Configure axes
    ax.set_xlim(data.limits.p1_min, data.limits.p1_max)
    ax.set_ylim(data.limits.p2_min, data.limits.p2_max)
    ax.set_xlabel(f'${data.parameters[0]}$')
    ax.set_ylabel(f'${data.parameters[1]}$')
    ax.xaxis.labelpad, ax.yaxis.labelpad = 0, 0

    configure_ticks(ax, cfg)

    # Fetch pregions of interest
    region = data.regions[1]
    pregion = region.pregions[0]
    add_pregions_to_ax(fig, ax, [pregion], data.limits, PREGION_COLOR)

    # Add origin
    ax.plot(region.origin[0], region.origin[1], 'x', color='black', markersize=3)

    # Add varepsilon_q
    eps = pregion.radius
    ax.plot([10, 10+eps], [10, 10], ':', linewidth=0.8, color='black')
    annotation = r'$\overline{\varepsilon}_{p, q}$'
    ax.annotate(annotation, [10+eps/2, 10], textcoords='offset points',
                xytext=(0, 5) , ha='center')
    logging.info(f'Drawing varepsilon_q = {eps}...')

    # Draw the legend
    legend_handles = [
        Line2D(
            [0], [0],
            color='black',
            linestyle='None',
            markersize=3,
            marker='x',
            label=r'Početna tačka $\eta^0$',
        ),
        Line2D(
            [0], [0],
            color=PREGION_COLOR,
            label=r'Dobijena oblast $\mathcal{S}_1$',
            linewidth=8,
        )
    ]
    ax.legend(handles=legend_handles, loc='upper left', frameon=False,
              bbox_to_anchor=cfg.bbox, mode='expand', ncol=cfg.ncol)
    tick_spacing = 5
    ax.xaxis.set_major_locator(ticker.MultipleLocator(tick_spacing))
    ax.yaxis.set_major_locator(ticker.MultipleLocator(tick_spacing))

    # Save fig
    dirname = f'output/custom'
    dir = Path(dirname)
    dir.mkdir(exist_ok=True, parents=True)
    figpath = f'{dirname}/{args.customscript}.pdf'
    fig.savefig(figpath, dpi=1000)


def instructional_region_nsc(args):
    """Draw step-by-step evolution of the region fsals algorithm."""
    data = storage.read_data_from_path(f'output/data/region/pde_complex_instructional.data')
    cfg = RegionConfiguration(
        width=4.7747,
        height=4.7747 / 3 * 4.2,
        ncol=2,
        ticks=TickConfiguration(5, 5, 5, 5),
        bbox=(0, -0.19, 1, 0.1),
    )
    COLOR = 'lightsteelblue'
    COLOR_LAST = 'lightcoral'

    set_general_parameters()

    # Fetch figure
    size = (cfg.width, cfg.height)
    rows, cols = 4, 3
    # rows, cols = 2, 3 # Uncomment for more efficient testing
    plot_count = rows * cols
    fig, axes = plt.subplots(rows, cols, figsize=size, constrained_layout=True)

    # Fetch region of interest
    region = data.regions[1]

    # # Configure axes
    k = 0
    for r in range(rows):
        for c in range(cols):
            k += 1
            ax = axes[r][c]
            ax.set_title(f'$k = {k}$', fontsize=8)
            ax.set_xlim(data.limits.p1_min, data.limits.p1_max)
            ax.set_ylim(data.limits.p2_min, data.limits.p2_max)

            configure_ticks(ax, cfg)

            pregions = [p for p in region.pregions if p.depth <= k]
            origins = False
            add_pregions_to_ax(fig, ax, pregions, data.limits, COLOR, COLOR_LAST, origins)
            if k == 1:
                x, y = region.origin[0], region.origin[1]
                ax.plot(x, y, 'x', color='black', markersize=2)

    # Save fig
    dirname = f'output/custom'
    dir = Path(dirname)
    dir.mkdir(exist_ok=True, parents=True)
    figpath = f'{dirname}/{args.customscript}.pdf'
    fig.savefig(figpath, dpi=1000)


def iterative_telegrapher_alpha_gamma_nsc(args):
    """
        Draw step-by-step evolution of the region fsals algorithm
        for the telegrapher_alpha_gamma example.
    """
    data_filename="telegrapher_alpha_gamma_single_region.data"
    figure_filename="telegrapher_alpha_gamma_iterative.pdf"
    color='darkred'
    create_iterative_region_figure(
        data_filename,
        figure_filename,
        region_index=0,
        rows=4,
        cols=3,
        color=color,
        fontsize=8,
        # xlabel=r'$\alpha$',
        # ylabel=r'$\gamma$',
        k_start=None,
        k_step=None,
        k_list=[1, 3, 5, 7, 9, 11, 13, 20, 25, 30, 35, 500],
    )


def create_iterative_region_figure(
        data_filename,
        figure_filename,
        region_index=1,
        rows=4,
        cols=3,
        color='lightsteelblue',
        fontsize=8,
        xlabel='',
        ylabel='',
        k_start=1,
        k_step=1,
        k_list=None,
        single_origin=False,
    ):
    """Draw step-by-step evolution of the region fsals algorithm for given example."""
    datapath = f'output/data/region/{data_filename}'
    data = storage.read_data_from_path(datapath)
    cfg = RegionConfiguration(
        width=4.7747,
        height=4.7747 / 3 * 4.2,
        # height=3.486429134 * 1.05,
        ncol=2,
        bbox=(0, -0.19, 1, 0.1),
    )

    set_general_parameters()

    # Fetch figure
    size = (cfg.width, cfg.height)
    fig, axes = plt.subplots(rows, cols, figsize=size, constrained_layout=True)

    # Fetch region of interest
    region = data.regions[region_index]

    # Prepare iteration counter (probably could be done better via generators)
    if k_list:
        i = -1
    else:
        k = k_start - k_step

    # Configure axes
    for row in range(rows):
        for column in range(cols):
            # Update iteration counter
            if k_list:
                i += 1
                k = k_list[i]
            else:
                k += k_step

            ax = axes[row][column]
            ax.set_title(f'$k = {k}$', fontsize=fontsize)
            ax.set_xlim(data.limits.p1_min, data.limits.p1_max)
            ax.set_ylim(data.limits.p2_min, data.limits.p2_max)
            ax.set_xlabel(xlabel)
            ax.set_ylabel(ylabel)

            configure_ticks(ax, cfg)

            pregions = [p for p in region.pregions if p.depth <= k]
            add_pregions_to_ax(fig, ax, pregions, data.limits, color)

            # Add origin(s)
            if single_origin:
                ax.plot(region.origin[0], region.origin[1], 'x', color='black', markersize=3)
            else:
                pass


    # Save fig
    dirname = f'output/custom/iterative_region'
    dir = Path(dirname)
    dir.mkdir(exist_ok=True, parents=True)
    figpath = f'{dirname}/{figure_filename}'
    fig.savefig(figpath, dpi=1000)


def sample_p_norm_boundary(p, N, eta, eps):
    # Make sure p is a number greater than or equal to 1 or infinity
    assert p >= 1 or np.isinf(p), "p must be greater than or equal to 1 or infinity"
    # Make sure N is a positive integer
    assert isinstance(N, int) and N > 0, "N must be a positive integer"
    # Make sure eta is a numpy array with exactly two elements
    assert isinstance(eta, np.ndarray) and eta.shape == (2,), "eta must be a numpy array with exactly two elements"

    if np.isinf(p):
        # In case of infinity norm, sample the boundary of a square.
        # Split N into 4 equal parts for the 4 edges of the square
        edge_points = N // 4
        # Generate points on each edge of the square
        x_left = eta[0] - np.ones(edge_points) * eps
        y_left = eta[1] + np.linspace(-eps, eps, edge_points)
        x_right = eta[0] + np.ones(edge_points) * eps
        y_right = eta[1] + np.linspace(eps, -eps, edge_points)
        x_top = eta[0] + np.linspace(-eps, eps, edge_points)
        y_top = eta[1] + np.ones(edge_points) * eps
        x_bottom = eta[0] + np.linspace(eps, -eps, edge_points)
        y_bottom = eta[1] - np.ones(edge_points) * eps
        # Concatenate points from all edges
        x = np.concatenate([x_left, x_top, x_right, x_bottom])
        y = np.concatenate([y_left, y_top, y_right, y_bottom])
    else:
        # Generate N equally spaced angles between 0 and 2pi
        theta = np.linspace(0, 2*np.pi, N)
        # Compute x and y coordinates for each theta
        x = eta[0] + eps * np.sign(np.cos(theta)) * (np.abs(np.cos(theta)) ** (2/p))
        y = eta[1] + eps * np.sign(np.sin(theta)) * (np.abs(np.sin(theta)) ** (2/p))

    return x, y


def w_ball(args):
    """Draw example W balls."""
    cfg = RegionConfiguration(
        width=4.7747,
        height=4.7747 * 1.05,
        ncol=2,
        ticks=TickConfiguration(5, 5, 5, 5),
        bbox=(0, -0.19, 1, 0.1),
    )
    COLOR = 'lightsteelblue'
    COLOR_LAST = 'lightcoral'

    set_general_parameters()

    # Fetch figure
    size = (cfg.width, cfg.height)
    rows, cols = 2, 2
    plot_count = rows * cols
    fig, axes = plt.subplots(rows, cols, figsize=size, constrained_layout=True)

    balls = [
        {
            'q' : 1,
            'eps' : 2,
        },
        {
            'q' : 2,
            'eps' : 4,
        },
        {
            'q' : 4,
            'eps' : 3,
        },
        {
            'q' : np.inf,
            'eps' : 2,
        },
    ]
    xmin, xmax = 0, 10
    ymin, ymax = 0, 10
    N = 64
    eta = np.array([5, 5])
    LABEL_OFFSET = (-3,-8)

    # Configure axes
    k = 0
    for r in range(rows):
        for c in range(cols):
            ax = axes[r][c]
            ball = balls[k]
            q, eps = ball['q'], ball['eps']

            if q == np.inf:
                q_plot = '\\infty'
            else:
                q_plot = str(q)

            ax.set_title(f'$\\eta=[5,5]$, $q={q_plot}$, $\\varepsilon={eps}$', fontsize=8)
            ax.set_xlim(xmin, xmax)
            ax.set_ylim(ymin, ymax)
            configure_ticks(ax, cfg)
            x, y = sample_p_norm_boundary(ball['q'], N, eta, eps)
            ax.fill(x, y)
            ax.plot([5], [5], 'x', color='black', markersize=3)
            k += 1
            ax.annotate(
                r'$\eta$',
                [5, 5],
                textcoords='offset points',
                xytext=LABEL_OFFSET,
                ha='right'
            )


    # Save fig
    dirname = f'output/custom'
    dir = Path(dirname)
    dir.mkdir(exist_ok=True, parents=True)
    figpath = f'{dirname}/{args.customscript}.pdf'
    fig.savefig(figpath, dpi=1000)


def main(args):
    """Run a python function pre-written to serve a non-generic fsals purpose."""
    logger.info(f'Running custom script {args.customscript}!')

    custom_scripts = {
        'instructional_line_sufficient' : instructional_line_sufficient,
        'instructional_line_nsc' : instructional_line_nsc,
        'instructional_line_nsc_multiple' : instructional_line_nsc_multiple,
        'instructional_region_sufficient' : instructional_region_sufficient,
        'instructional_region_nsc' : instructional_region_nsc,
        'iterative_telegrapher_alpha_gamma_nsc' : iterative_telegrapher_alpha_gamma_nsc,
        'w_ball' : w_ball,
    }

    if args.customscript in custom_scripts:
        custom_scripts[args.customscript](args)
    else:
        print(f'Error: Unknown custom script {args.customscript}')
        print(f'Possible scripts:')
        for script in custom_scripts:
            print(f'  {script}')
        raise Exception(f'Unknown custom script {args.customscript}')