from matplotlib.lines import Line2D

import python.utils.comparison_methods.gu2005 as gu2005


SCALING_FACTOR = 1.1
SINGLE_COLUMN_WIDTH = 8.85553
DOUBLE_COLUMN_WIDTH = 18.3436
STANDARD_HEIGHT = SINGLE_COLUMN_WIDTH * SCALING_FACTOR


class TickConfiguration:
    __slots__ = ['major_x', 'major_y', 'minor_x', 'minor_y']

    def __init__(self, major_x, major_y, minor_x, minor_y):
        self.major_x = major_x
        self.major_y = major_y
        self.minor_x = minor_x
        self.minor_y = minor_y


def default_custom_func(ax, legend_handles):
    return ax, legend_handles


class FigureConfiguration:
    __slots__ = ['width', 'height', 'ticks', 'custom_func', 'ncol', 'bbox',
                 'draw_origins']

    def __init__(
        self, width, height, ncol, bbox, custom_func=default_custom_func,
        draw_origins=True, ticks=None):
        self.width = width
        self.height = height
        self.ticks = ticks
        self.custom_func = custom_func
        self.ncol = ncol
        self.bbox = bbox
        self.draw_origins = draw_origins


class LineConfiguration(FigureConfiguration):
    __slots__ = ['ratios']

    def __init__(
        self, width, height, ncol, bbox,
        ratios=None, custom_func=default_custom_func, draw_origins=True, ticks=None):

        super().__init__(width, height, ncol, bbox, custom_func, draw_origins,
            ticks=ticks)
        self.ratios = ratios


class RegionConfiguration(FigureConfiguration):
    __slots__ = []

    def __init__(self, width, height, ncol, bbox,
        custom_func=default_custom_func, draw_origins=True, ticks=None):
        super().__init__(width, height, ncol, bbox, custom_func, draw_origins,
            ticks=ticks)


def retarded1_custom_func(ax, legend_handles):
    gu2005.add_gu2005_example1(ax)
    legend_handle = Line2D([0], [0], color='black', linestyle='--', label='SCS')

    legend_handles.append(legend_handle)
    return ax, legend_handles


LINE_CONFIGURATIONS = {
    'retarded1' : LineConfiguration(
        width=SINGLE_COLUMN_WIDTH,
        height=SINGLE_COLUMN_WIDTH * SCALING_FACTOR,
        ticks=TickConfiguration(1, 1, 0.25, 0.25),
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        ratios=[1, 2, 5, 5, 8],
        custom_func=retarded1_custom_func,
    ),

    'distributed_delay1' : LineConfiguration(
        width=DOUBLE_COLUMN_WIDTH,
        height=SINGLE_COLUMN_WIDTH, # 3 is the magic number
        ticks=TickConfiguration(1, 1, 0.25, 0.25),
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
    ),
}

REGION_CONFIGURATIONS = {
    'retarded1' : RegionConfiguration(
        width=SINGLE_COLUMN_WIDTH,
        height=SINGLE_COLUMN_WIDTH * SCALING_FACTOR,
        ticks=TickConfiguration(1, 1, 0.25, 0.25),
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        custom_func=retarded1_custom_func,
    ),

    'distributed_delay1' : RegionConfiguration(
        width=DOUBLE_COLUMN_WIDTH,
        height=SINGLE_COLUMN_WIDTH, # 3 is the magic number
        ticks=TickConfiguration(1, 1, 0.25, 0.25),
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'semi_infinite_rod' : RegionConfiguration(
        width=SINGLE_COLUMN_WIDTH,
        height=STANDARD_HEIGHT,
        ncol=2,
        bbox=(0, -0.17, 1, 0.1),
    ),
}