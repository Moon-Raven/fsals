from matplotlib.lines import Line2D

import python.utils.comparison_methods.gu2005 as gu2005


# Factor used to get height as slightly larger than width
W2H_RATIO = 1.1    # Because extra legend rows destroy ratio
PDE_W2HRATIO = 1.1 # Because extra legend rows destroy ratio
ROD_W2HRATIO = 1.0 # Because there are no extra legend rows to destroy ratio
TELEGRAPHER_X_K_W2HRATIO = 1.0
TELEGRAPHER_ALPHA_GAMMA_W2HRATIO = 1.05
TELEGRAPHER_STANDARD_W2HRATIO = 1.1

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


def default_custom_func(ax, legend_handles, language='english'):
    return ax, legend_handles


class FigureConfiguration:
    __slots__ = ['width', 'height', 'ticks', 'custom_func', 'ncol', 'bbox',
                 'draw_origins', 'language', 'system']

    def __init__(
        self, system, width, height, ncol, bbox, custom_func=default_custom_func,
        draw_origins=True, ticks=None, language='english'
    ):
        self.system = system
        self.width = width
        self.height = height
        self.ticks = ticks
        self.custom_func = custom_func
        self.ncol = ncol
        self.bbox = bbox
        self.draw_origins = draw_origins
        self.language = language


class LineConfiguration(FigureConfiguration):
    __slots__ = ['ratios']

    def __init__(
        self, system, width, height, ncol, bbox, ratios=None,
        custom_func=default_custom_func, draw_origins=True, ticks=None, language='english'
    ):
        super().__init__(
            system, width, height, ncol, bbox, custom_func, draw_origins,
            ticks=ticks, language=language
        )
        self.ratios = ratios


class RegionConfiguration(FigureConfiguration):
    __slots__ = []

    def __init__(
        self, system, width, height, ncol, bbox, custom_func=default_custom_func,
        draw_origins=True, ticks=None, language='english'
    ):
        super().__init__(
            system, width, height, ncol, bbox, custom_func, draw_origins,
            ticks=ticks, language=language
        )


def retarded1_custom_func(ax, legend_handles, language='english'):
    SCS_LABELS = {'english': 'SCS', 'serbian': 'SPS'}
    gu2005.add_gu2005_example1(ax)
    legend_handle = Line2D(
        [0], [0],
        color='black',
        linestyle='--',
        label=SCS_LABELS[language],
    )
    legend_handles.append(legend_handle)
    return ax, legend_handles


LINE_CONFIGURATIONS = {
    'retarded1' : LineConfiguration(
        system='retarded1',
        width=COMMON_WIDTHS['single_column'],
        height=COMMON_WIDTHS['single_column'] * W2H_RATIO,
        ticks=TickConfiguration(1, 1, 0.25, 0.25),
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        ratios=[1, 2, 5, 5, 8],
        custom_func=retarded1_custom_func,
    ),

    'retarded1_thesis' : LineConfiguration(
        system='retarded1',
        width=COMMON_WIDTHS['single_column'],
        height=COMMON_WIDTHS['single_column'] * W2H_RATIO,
        ticks=TickConfiguration(1, 1, 0.25, 0.25),
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        ratios=[1, 2, 5, 5, 8],
        custom_func=retarded1_custom_func,
        language='serbian',
    ),

    'distributed_delay1' : LineConfiguration(
        system='distributed_delay1',
        width=COMMON_WIDTHS['double_column'],
        height=COMMON_WIDTHS['single_column'],
        ticks=TickConfiguration(1, 1, 0.25, 0.25),
        ncol=6,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'distributed_delay1_thesis' : LineConfiguration(
        system='distributed_delay1',
        width=4.7747,
        height=4.7747 / 3,
        ticks=TickConfiguration(2, 0.1, 1, 0.05),
        ncol=3,
        bbox=(0, -0.30, 1, 0.1),
        language='serbian',
    ),

    'pde_complex_k_sigma' : LineConfiguration(
        system='pde_complex_k_sigma',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * PDE_W2HRATIO,
        ncol=1,
        bbox=(0, -0.19, 1, 0.1),
    ),

    'pde_complex_beta_sigma' : LineConfiguration(
        system='pde_complex_beta_sigma',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * PDE_W2HRATIO,
        ncol=1,
        bbox=(0, -0.19, 1, 0.1),
    ),

    'pde_complex_tau_sigma' : LineConfiguration(
        system='pde_complex_tau_sigma',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * PDE_W2HRATIO,
        ncol=2,
        bbox=(0, -0.19, 1, 0.1),
    ),

    'pde_complex_tau_sigma_instructional' : LineConfiguration(
        system='pde_complex_tau_sigma_instructional',
        width=COMMON_WIDTHS['single_column'],
        height=COMMON_WIDTHS['single_column'] * PDE_W2HRATIO,
        language='serbian',
        ncol=2,
        bbox=(0, -0.19, 1, 0.1),
    ),

    'telegrapher_x_k' : LineConfiguration(
        system='telegrapher_x_k',
        width=COMMON_WIDTHS['double_subfigure'],
        height=COMMON_WIDTHS['double_subfigure'] * TELEGRAPHER_X_K_W2HRATIO,
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'telegrapher_alpha_gamma' : LineConfiguration(
        system='telegrapher_alpha_gamma',
        width=COMMON_WIDTHS['double_subfigure'],
        height=COMMON_WIDTHS['double_subfigure'] * TELEGRAPHER_ALPHA_GAMMA_W2HRATIO,
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'semi_infinite_rod' : LineConfiguration(
        system='semi_infinite_rod',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * ROD_W2HRATIO,
        ncol=2,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'finite_rod' : LineConfiguration(
        system='finite_rod',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * ROD_W2HRATIO,
        ncol=2,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'telegrapher_standard' : LineConfiguration(
        system='telegrapher_standard',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * ROD_W2HRATIO,
        ncol=2,
        bbox=(0, -0.17, 1, 0.1),
    ),
}

REGION_CONFIGURATIONS = {
    'retarded1' : RegionConfiguration(
        system='retarded1',
        width=COMMON_WIDTHS['single_column'],
        height=COMMON_WIDTHS['single_column'] * W2H_RATIO,
        ticks=TickConfiguration(1, 1, 0.25, 0.25),
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        custom_func=retarded1_custom_func,
    ),

    'retarded1_thesis' : RegionConfiguration(
        system='retarded1',
        width=COMMON_WIDTHS['single_column'],
        height=COMMON_WIDTHS['single_column'] * W2H_RATIO,
        ticks=TickConfiguration(1, 1, 0.25, 0.25),
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        custom_func=retarded1_custom_func,
        language='serbian',
    ),

    'distributed_delay1' : RegionConfiguration(
        system='distributed_delay1',
        width=COMMON_WIDTHS['double_column'],
        height=COMMON_WIDTHS['single_column'],
        ticks=TickConfiguration(1, 0.5, 0.25, 0.25),
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'pde_complex_k_sigma' : RegionConfiguration(
        system='pde_complex_k_sigma',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * PDE_W2HRATIO,
        ncol=1,
        bbox=(0, -0.19, 1, 0.1),
    ),

    'pde_complex_beta_sigma' : RegionConfiguration(
        system='pde_complex_beta_sigma',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * PDE_W2HRATIO,
        ncol=1,
        bbox=(0, -0.19, 1, 0.1),
    ),

    'pde_complex_tau_sigma' : RegionConfiguration(
        system='pde_complex_tau_sigma',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * PDE_W2HRATIO,
        ncol=2,
        bbox=(0, -0.19, 1, 0.1),
    ),

    'telegrapher_x_k' : RegionConfiguration(
        system='telegrapher_x_k',
        width=COMMON_WIDTHS['double_subfigure'],
        height=COMMON_WIDTHS['double_subfigure'] * TELEGRAPHER_X_K_W2HRATIO,
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
    ),

     'telegrapher_alpha_gamma' : RegionConfiguration(
        system='telegrapher_alpha_gamma',
        width=COMMON_WIDTHS['double_subfigure'],
        height=COMMON_WIDTHS['double_subfigure'] * TELEGRAPHER_ALPHA_GAMMA_W2HRATIO,
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'semi_infinite_rod' : RegionConfiguration(
        system='semi_infinite_rod',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * ROD_W2HRATIO,
        ncol=2,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'finite_rod' : RegionConfiguration(
        system='finite_rod',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * ROD_W2HRATIO,
        ncol=2,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'telegrapher_standard' : RegionConfiguration(
        system='telegrapher_standard',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * ROD_W2HRATIO,
        ncol=2,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'pde_complex_instructional' : RegionConfiguration(
        system='pde_complex_tau_sigma',
        width=COMMON_WIDTHS['single_column'],
        height=COMMON_WIDTHS['single_column'] * PDE_W2HRATIO,
        ncol=2,
        bbox=(0, -0.19, 1, 0.1),
        language='serbian'
    ),
}