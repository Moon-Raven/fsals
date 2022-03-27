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


logger = logging.getLogger(__name__)

ORIGIN_MARKERSTYLE = 'X'
ORIGIN_MARKERCOLOR = 'black'
ORIGIN_MARKERSIZE = 4
ORIGIN_LABEL_LINE = 'Origins'
ORIGIN_LABEL_REGION = 'Origins'


def set_general_parameters():
    mpl.rcParams['font.family'] = 'serif'
    mpl.rcParams['font.serif'] = ['Computer Modern Roman']
    mpl.rcParams['text.usetex'] = True
    mpl.rcParams['axes.labelsize'] = 8
    mpl.rcParams['xtick.labelsize'] = 8
    mpl.rcParams['ytick.labelsize'] = 8
    mpl.rcParams['legend.fontsize'] = 7


def new_figure_inches(width, height, tight=True, constrained_layout=False):
    size = (width, height)
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

    if cfg.ticks:
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

    for ray in rayfan.rays[0::ratio]:
        add_ray_to_ax(ax, ray, linecolor, linewidth)

    if origins:
        p1 = rayfan.origin[0]
        p2 = rayfan.origin[1]
        ax.plot(p1, p2, ORIGIN_MARKERSTYLE, color=ORIGIN_MARKERCOLOR,
                clip_on=False, markersize=ORIGIN_MARKERSIZE)


def create_figure_line(args):
    data = read_data(args)
    cfg = LINE_CONFIGURATIONS[args.system]

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
    ax.xaxis.labelpad = 0
    ax.yaxis.labelpad = 0
    configure_ticks(ax, cfg)

    linewidth = 1
    colors = {0: 'g', 2: 'darkred', 4: 'cornflowerblue', 6: 'orange', 8: 'mediumpurple'}
    nus = set()

    if cfg.ratios == None:
        ratios = [1] * len(data.rayfans)
    else:
        ratios = cfg.ratios

    for rayfan, ratio in zip(data.rayfans, ratios):
        nu = rayfan.nu
        nus.add(nu)
        color = colors[nu]
        logging.debug(f'Taking color {color} for nu {nu}')
        add_rayfan_to_ax(ax, rayfan, color, linewidth, ratio, cfg.draw_origins)

    # Prepare regular legend labels
    nus = sorted(list(nus))
    legend_handles = [
        Line2D([0], [0], color=colors[nu], label=f'$NU_f$ = {nu}') for nu in nus]

    # Add origin label, if necessary
    if cfg.draw_origins:
        origin_handle = Line2D([0], [0], color='black', linestyle='None',
            markersize=4, marker='X', label=ORIGIN_LABEL_LINE),
        legend_handles.insert(0, *origin_handle)


    # Call custom drawing actions for given system
    ax, legend_handles = cfg.custom_func(ax, legend_handles)

    # Draw the legend
    ax.legend(handles=legend_handles, loc='upper left', frameon=False,
              bbox_to_anchor=cfg.bbox, mode='expand', ncol=cfg.ncol)

    return fig


def get_ax_ratio(fig, ax):
    """Get heigth / width ratio of given axes"""
    bbox = ax.get_window_extent().transformed(fig.dpi_scale_trans.inverted())
    width_inch, height_inch = bbox.width, bbox.height
    return height_inch / width_inch


def get_nus(regions):
    nus = set()
    for region in regions:
        nus.add(region.nu)
    return nus


def get_image_dimensions(desired_ratio):
    width = 5000 # pixels
    height = round(width * desired_ratio)
    return width, height


def get_drawable_canvas(width, height):
    WHITE = (255, 255, 255)
    img  = Image.new(mode = "RGB", size = (width, height), color=WHITE)
    drawable = ImageDraw.Draw(img)
    return img, drawable


def add_origins_to_ax(ax, regions):
    for region in regions:
        add_origin_to_ax(ax, region.origin)


def get_corners(pregion):
    upper_left_p1 = pregion.origin[0] - pregion.radius
    upper_left_p2 = pregion.origin[1] - pregion.radius
    lower_right_p1 = pregion.origin[0] + pregion.radius
    lower_right_p2 = pregion.origin[1] + pregion.radius

    upper_left = np.array([upper_left_p1, upper_left_p2])
    lower_right = np.array([lower_right_p1, lower_right_p2])

    return upper_left, lower_right


def corners2pixels(corners, spans, pixel_dimensions, mins):
    upper, lower = corners

    upper_ratios = (upper - mins) / spans
    lower_ratios = (lower - mins) / spans

    upper_pixels = np.rint(upper_ratios * pixel_dimensions).astype(int)
    lower_pixels = np.rint(lower_ratios * pixel_dimensions).astype(int)

    return upper_pixels, lower_pixels


def add_regions_to_ax(fig, ax, data, colors):
    ratio = get_ax_ratio(fig, ax)
    width, height = get_image_dimensions(ratio)
    pixel_dimensions = np.array([width, height])

    image, canvas = get_drawable_canvas(width, height)
    p1span = data.limits.p1_max - data.limits.p1_min
    p2span = data.limits.p2_max - data.limits.p2_min
    spans = np.array([p1span, p2span])
    mins = np.array([data.limits.p1_min, data.limits.p2_min])

    for region in data.regions:
        color = mpl.colors.to_rgb(colors[region.nu])
        color = tuple([round(c*255) for c in color])

        for pregion in region.pregions:
            corners = get_corners(pregion)
            upper_np, lower_np = corners2pixels(corners, spans, pixel_dimensions, mins)
            upper, lower = (upper_np[0], upper_np[1]), (lower_np[0], lower_np[1])
            canvas.ellipse([upper, lower], fill=color)

    box = [data.limits.p1_min, data.limits.p1_max, data.limits.p2_min, data.limits.p2_max]
    ax.imshow(image, extent=box, aspect='auto', origin='lower')



def get_pregion_boundary(pregion, N=1000, p=2):
    """Spawn an array of points on pregion's boundary."""
    x1 = np.linspace(-pregion.radius, pregion.radius, N)
    if p == np.inf:
        x2 = np.where(x1<pregion.radius, pregion.radius, 0)
    else:
        x2 = np.power(np.power(pregion.radius, p) - np.power(np.abs(x1), p), 1/p)
    x1_total = np.concatenate((x1, np.flip(x1.copy())))
    x2_total = np.concatenate((x2, np.flip(-x2.copy())))
    unshifted_boundary = np.vstack((x1_total, x2_total))
    shifted_boundary = unshifted_boundary + np.array(pregion.origin, ndmin=2).T
    return shifted_boundary


def add_pregion_to_ax(ax, pregion, color, fill=True):
    """Draw pregion object to given axes."""
    region_boundary = get_pregion_boundary(pregion)
    add_polygon(ax, region_boundary, color, fill)


def add_origin_to_ax(ax, origin):
    ax.plot(origin[0], origin[1], ORIGIN_MARKERSTYLE, color=ORIGIN_MARKERCOLOR,
            clip_on=False, markersize=ORIGIN_MARKERSIZE)


def add_polygon(ax, poly_boundary, style_string='g', fill=True):
    """Draw polygon to given axes."""
    if fill:
        ax.fill(poly_boundary[0,:], poly_boundary[1,:], style_string, rasterized=True)
    else:
        ax.plot(poly_boundary[0,:], poly_boundary[1,:], style_string)


def create_figure_region(args):
    data = read_data(args)
    cfg = REGION_CONFIGURATIONS[args.system]
    colors = {0: 'g', 2: 'darkred', 4: 'cornflowerblue', 6: 'orange', 8: 'mediumpurple'}

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

    add_regions_to_ax(fig, ax, data, colors)
    nus = get_nus(data.regions)

    if cfg.draw_origins:
        add_origins_to_ax(ax, data.regions)

    # Prepare regular legend labels
    nus = sorted(list(nus))
    legend_handles = [
        Line2D([0], [0], color=colors[nu], label=f'$NU_f$ = {nu}') for nu in nus]

    # Add origin label, if necessary
    if cfg.draw_origins:
        origin_handle = Line2D([0], [0], color='black', linestyle='None',
            markersize=4, marker='X', label=ORIGIN_LABEL_REGION),
        legend_handles.insert(0, *origin_handle)

    # Call custom drawing actions for given system
    ax, legend_handles = cfg.custom_func(ax, legend_handles)

    # Draw the legend
    ax.legend(handles=legend_handles, loc='upper left', frameon=False,
              bbox_to_anchor=cfg.bbox, mode='expand', ncol=cfg.ncol)

    return fig


def main(args):
    if args.algorithm == 'line':
        fig = create_figure_line(args)
        extension = 'pdf'
        storage.save_figure(args, fig, args.command, args.algorithm, extension)
    elif args.algorithm == 'region':
        fig =  create_figure_region(args)
        extension = 'pdf'
        storage.save_figure(args, fig, args.command, args.algorithm, extension)
    else:
        raise Exception(f'Unknown algorithm: {args.algorithm}')
