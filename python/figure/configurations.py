from matplotlib.lines import Line2D

import python.utils.comparison_methods.gu2005 as gu2005


# Factor used to get height as slightly larger than width
W2H_RATIO = 1.1    # Because extra legend rows destroy ratio
PDE_W2HRATIO = 1.1 # Because extra legend rows destroy ratio
ROD_W2HRATIO = 1.0 # Because there are no extra legend rows to destroy ratio
TELEGRAPHER_X_K_W2HRATIO = 1.0
TELEGRAPHER_ALPHA_GAMMA_W2HRATIO = 1.05

# Commonly used figure widths, in inches
COMMON_WIDTHS = {
    'single_column': 3.486429134,
    'double_column': 7.221889764,
    'triple_subfigure': 2.38403,
    'double_subfigure': 3.61217,
}


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
        width=COMMON_WIDTHS['single_column'],
        height=COMMON_WIDTHS['single_column'] * W2H_RATIO,
        ticks=TickConfiguration(1, 1, 0.25, 0.25),
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        ratios=[1, 2, 5, 5, 8],
        custom_func=retarded1_custom_func,
    ),

    'distributed_delay1' : LineConfiguration(
        width=COMMON_WIDTHS['double_column'],
        height=COMMON_WIDTHS['single_column'], # Eyeballed and inelegant
        ticks=TickConfiguration(1, 1, 0.25, 0.25),
        ncol=6,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'pde_complex_k_sigma' : LineConfiguration(
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * PDE_W2HRATIO,
        ncol=1,
        bbox=(0, -0.19, 1, 0.1),
    ),

    'pde_complex_beta_sigma' : LineConfiguration(
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * PDE_W2HRATIO,
        ncol=1,
        bbox=(0, -0.19, 1, 0.1),
    ),

    'pde_complex_tau_sigma' : LineConfiguration(
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * PDE_W2HRATIO,
        ncol=2,
        bbox=(0, -0.19, 1, 0.1),
    ),

    'telegrapher_x_k' : LineConfiguration(
        width=COMMON_WIDTHS['double_subfigure'],
        height=COMMON_WIDTHS['double_subfigure'] * TELEGRAPHER_X_K_W2HRATIO,
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'telegrapher_alpha_gamma' : LineConfiguration(
        width=COMMON_WIDTHS['double_subfigure'],
        height=COMMON_WIDTHS['double_subfigure'] * TELEGRAPHER_ALPHA_GAMMA_W2HRATIO,
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'semi_infinite_rod' : LineConfiguration(
        width=COMMON_WIDTHS['double_subfigure'],
        height=COMMON_WIDTHS['double_subfigure'] * ROD_W2HRATIO,
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'finite_rod' : LineConfiguration(
        width=COMMON_WIDTHS['double_subfigure'],
        height=COMMON_WIDTHS['double_subfigure'] * ROD_W2HRATIO,
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
    ),
}

REGION_CONFIGURATIONS = {
    'retarded1' : RegionConfiguration(
        width=COMMON_WIDTHS['single_column'],
        height=COMMON_WIDTHS['single_column'] * W2H_RATIO,
        ticks=TickConfiguration(1, 1, 0.25, 0.25),
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        custom_func=retarded1_custom_func,
    ),

    'distributed_delay1' : RegionConfiguration(
        width=COMMON_WIDTHS['double_column'],
        height=COMMON_WIDTHS['single_column'], # Eyeballed and inelegant
        ticks=TickConfiguration(1, 1, 0.25, 0.25),
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'pde_complex_k_sigma' : RegionConfiguration(
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * PDE_W2HRATIO,
        ncol=1,
        bbox=(0, -0.19, 1, 0.1),
    ),

    'pde_complex_beta_sigma' : RegionConfiguration(
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * PDE_W2HRATIO,
        ncol=1,
        bbox=(0, -0.19, 1, 0.1),
    ),

    'pde_complex_tau_sigma' : RegionConfiguration(
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * PDE_W2HRATIO,
        ncol=2,
        bbox=(0, -0.19, 1, 0.1),
    ),

    'telegrapher_x_k' : RegionConfiguration(
        width=COMMON_WIDTHS['double_subfigure'],
        height=COMMON_WIDTHS['double_subfigure'] * TELEGRAPHER_X_K_W2HRATIO,
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
    ),

     'telegrapher_alpha_gamma' : RegionConfiguration(
        width=COMMON_WIDTHS['double_subfigure'],
        height=COMMON_WIDTHS['double_subfigure'] * TELEGRAPHER_ALPHA_GAMMA_W2HRATIO,
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'semi_infinite_rod' : RegionConfiguration(
        width=COMMON_WIDTHS['double_subfigure'],
        height=COMMON_WIDTHS['double_subfigure'] * ROD_W2HRATIO,
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
    ),

     'finite_rod' : RegionConfiguration(
        width=COMMON_WIDTHS['double_subfigure'],
        height=COMMON_WIDTHS['double_subfigure'] * ROD_W2HRATIO,
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
    ),
}