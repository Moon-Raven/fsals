"""Configurations for plotting fsals results on matplotlib figures."""
from matplotlib.lines import Line2D

import python.utils.comparison_methods.gu2005 as gu2005


# Factors used to get height as slightly larger than width
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
    'thesis_standard': 3.256429134,
    'thesis_textwidth': 4.9823,
}


class TickConfiguration:
    """Container for major and minor tick parameters of a matplotlib axes."""
    __slots__ = ['major_x', 'major_y', 'minor_x', 'minor_y']

    def __init__(self, major_x, major_y, minor_x, minor_y):
        self.major_x = major_x
        self.major_y = major_y
        self.minor_x = minor_x
        self.minor_y = minor_y


def default_custom_func(ax, legend_handles, language='english'):
    """Dummy custom axes modifier function (no behavior)."""
    return ax, legend_handles


class FigureConfiguration:
    """Parameters for creating a fsals results figure."""
    __slots__ = ['width', 'height', 'ticks', 'custom_func', 'ncol', 'bbox',
                 'draw_origins', 'language', 'rust_configuration']

    def __init__(
        self, width, height, ncol, bbox,
        rust_configuration=None,
        custom_func=default_custom_func,
        draw_origins=True,
        ticks=None,
        language='english',
    ):
        self.rust_configuration = rust_configuration
        self.width = width
        self.height = height
        self.ticks = ticks
        self.custom_func = custom_func
        self.ncol = ncol
        self.bbox = bbox
        self.draw_origins = draw_origins
        self.language = language


class LineConfiguration(FigureConfiguration):
    """Parameters for creating a line fsals results figure."""
    __slots__ = ['ratios']

    def __init__(
        self, width, height, ncol, bbox,
        rust_configuration=None,
        ratios=None,
        custom_func=default_custom_func,
        draw_origins=True,
        ticks=None,
        language='english'
    ):
        super().__init__(
            width, height, ncol, bbox,
            rust_configuration=rust_configuration,
            custom_func=custom_func,
            draw_origins=draw_origins,
            ticks=ticks,
            language=language,
        )
        self.ratios = ratios


class RegionConfiguration(FigureConfiguration):
    """Parameters for creating a region fsals results figure."""
    __slots__ = ['ratios']
    __slots__ = []

    def __init__(
        self, width, height, ncol, bbox,
        rust_configuration=None,
        custom_func=default_custom_func,
        draw_origins=True,
        ticks=None,
        language='english',
    ):
        super().__init__(
            width, height, ncol, bbox,
            rust_configuration=rust_configuration,
            custom_func=custom_func,
            draw_origins=draw_origins,
            ticks=ticks,
            language=language,
        )


def retarded1_custom_func(ax, legend_handles, language='english'):
    """Add results from Gu2005 to given axes."""
    SCS_LABELS = {'english': 'SCS', 'serbian': 'SCS'}
    gu2005.add_gu2005_example1(ax)
    legend_handle = Line2D(
        [0], [0],
        color='black',
        linestyle='--',
        label=SCS_LABELS[language],
    )
    legend_handles.append(legend_handle)
    return ax, legend_handles


def distributed_delay1_narrow_custom_func(ax, legend_handles, language='english'):
    """Cut off portion of axes in order to hide uninteresting areas."""
    ax.set_xlim(0, 20.0)
    ax.set_ylim(0, 0.30)
    return ax, legend_handles


LINE_CONFIGURATIONS = {
    'retarded1' : LineConfiguration(
        rust_configuration='retarded1',
        width=COMMON_WIDTHS['single_column'],
        height=COMMON_WIDTHS['single_column'] * W2H_RATIO,
        ticks=TickConfiguration(1, 1, 0.25, 0.25),
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        ratios=[1, 2, 5, 5, 8],
        custom_func=retarded1_custom_func,
    ),

    'retarded1_thesis' : LineConfiguration(
        rust_configuration='retarded1',
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'] * W2H_RATIO,
        ticks=TickConfiguration(1, 1, 0.25, 0.25),
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        ratios=[1, 2, 5, 5, 8],
        custom_func=retarded1_custom_func,
        language='serbian',
    ),

    'distributed_delay1' : LineConfiguration(
        rust_configuration='distributed_delay1',
        width=COMMON_WIDTHS['double_column'],
        height=COMMON_WIDTHS['single_column'],
        ticks=TickConfiguration(1, 1, 0.25, 0.25),
        ncol=6,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'distributed_delay1_thesis' : LineConfiguration(
        rust_configuration='distributed_delay1',
        width=COMMON_WIDTHS['thesis_textwidth'],
        height=COMMON_WIDTHS['thesis_textwidth'] / 3,
        ticks=TickConfiguration(2, 0.1, 1, 0.05),
        ncol=3,
        bbox=(0, -0.30, 1, 0.1),
        language='serbian',
    ),

    'pde_complex_k_sigma' : LineConfiguration(
        rust_configuration='pde_complex_k_sigma',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * PDE_W2HRATIO,
        ncol=1,
        bbox=(0, -0.19, 1, 0.1),
    ),

    'pde_complex_k_sigma_thesis' : LineConfiguration(
        rust_configuration='pde_complex_k_sigma',
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'] * 1.05,
        ticks=TickConfiguration(5, 5, 1, 1),
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        language='serbian',
    ),

    'pde_complex_beta_sigma' : LineConfiguration(
        rust_configuration='pde_complex_beta_sigma',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * PDE_W2HRATIO,
        ncol=1,
        bbox=(0, -0.19, 1, 0.1),
    ),

    'pde_complex_beta_sigma_thesis' : LineConfiguration(
        rust_configuration='pde_complex_beta_sigma',
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'],
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        language='serbian',
    ),

    'pde_complex_tau_sigma' : LineConfiguration(
        rust_configuration='pde_complex_tau_sigma',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * PDE_W2HRATIO,
        ncol=2,
        bbox=(0, -0.19, 1, 0.1),
    ),

    'pde_complex_tau_sigma_instructional' : LineConfiguration(
        rust_configuration='pde_complex_tau_sigma_instructional',
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'] * 1.15,
        language='serbian',
        ncol=2,
        bbox=(0, -0.19, 1, 0.1),
        ticks=TickConfiguration(5, 5, 1, 1),
    ),

    'telegrapher_x_k' : LineConfiguration(
        rust_configuration='telegrapher_x_k',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * 1.05,
        ncol=2,
        bbox=(0, -0.17, 1, 0.1),
        ratios = [4, 4, 4, 4, 4],
    ),

    'telegrapher_x_k_thesis' : LineConfiguration(
        rust_configuration='telegrapher_x_k',
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'] * 1.05,
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        language='serbian',
    ),

    'telegrapher_alpha_gamma' : LineConfiguration(
        rust_configuration='telegrapher_alpha_gamma',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * 1.15,
        ncol=2,
        bbox=(0, -0.17, 1, 0.1),
        ratios = [2, 2, 4, 8],
    ),

    'telegrapher_alpha_gamma_thesis' : LineConfiguration(
        rust_configuration='telegrapher_alpha_gamma',
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'] * 1.05,
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        language='serbian',
    ),

    'semi_infinite_rod' : LineConfiguration(
        rust_configuration='semi_infinite_rod',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * ROD_W2HRATIO,
        ncol=2,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'semi_infinite_rod_thesis' : LineConfiguration(
        rust_configuration='semi_infinite_rod',
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'],
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        language='serbian',
    ),

    'finite_rod' : LineConfiguration(
        rust_configuration='finite_rod',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * ROD_W2HRATIO,
        ncol=2,
        bbox=(0, -0.17, 1, 0.1),
        ratios=[4, 4, 4, 4],
    ),

    'finite_rod_thesis' : LineConfiguration(
        rust_configuration='finite_rod',
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'],
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        language='serbian',
    ),

    'telegrapher_standard' : LineConfiguration(
        rust_configuration='telegrapher_standard',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * ROD_W2HRATIO,
        ncol=2,
        bbox=(0, -0.17, 1, 0.1),
        ratios=[4, 4, 6, 6, 8],
    ),

    'telegrapher_standard_thesis' : LineConfiguration(
        rust_configuration='telegrapher_standard',
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'],
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        ticks=TickConfiguration(0.05, 0.1, 0.05, 0.1),
        language='serbian',
    ),

    'test_configuration' : LineConfiguration(
        rust_configuration='test_configuration',
        ncol=3,
        width=COMMON_WIDTHS['single_column'],
        height=COMMON_WIDTHS['single_column'] * W2H_RATIO,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'ln_system1' : LineConfiguration(
        rust_configuration='ln_system1',
        ncol=3,
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'],
        bbox=(0, -0.17, 1, 0.1),
        ticks=TickConfiguration(5, 5, 1, 1),
    ),

    'ln_system1_negative' : LineConfiguration(
        rust_configuration='ln_system1_negative',
        ncol=3,
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'],
        bbox=(0, -0.17, 1, 0.1),
        ticks=TickConfiguration(5, 5, 1, 1),
    ),
}


REGION_CONFIGURATIONS = {
    'retarded1' : RegionConfiguration(
        rust_configuration='retarded1',
        width=COMMON_WIDTHS['single_column'],
        height=COMMON_WIDTHS['single_column'] * W2H_RATIO,
        ticks=TickConfiguration(1, 1, 0.25, 0.25),
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        custom_func=retarded1_custom_func,
    ),

    'retarded2' : RegionConfiguration(
        rust_configuration='retarded2',
        width=COMMON_WIDTHS['single_column'],
        height=COMMON_WIDTHS['single_column'] * W2H_RATIO,
        ticks=TickConfiguration(1, 1, 0.25, 0.25),
        ncol=2,
        bbox=(0, -0.17, 1, 0.1),
    ),


    'retarded1_thesis' : RegionConfiguration(
        rust_configuration='retarded1',
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'] * W2H_RATIO,
        ticks=TickConfiguration(1, 1, 0.25, 0.25),
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        custom_func=retarded1_custom_func,
        language='serbian',
    ),

    'distributed_delay1' : RegionConfiguration(
        rust_configuration='distributed_delay1',
        width=COMMON_WIDTHS['double_column'],
        height=COMMON_WIDTHS['single_column'],
        ticks=TickConfiguration(1, 0.5, 0.25, 0.25),
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'distributed_delay1_narrow' : RegionConfiguration(
        rust_configuration='distributed_delay1',
        width=COMMON_WIDTHS['single_column'],
        height=COMMON_WIDTHS['single_column'] * W2H_RATIO,
        ticks=TickConfiguration(5, 0.1, 1, 0.05),
        ncol=3,
        custom_func=distributed_delay1_narrow_custom_func,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'distributed_delay1_thesis' : RegionConfiguration(
        rust_configuration='distributed_delay1',
        width=4.7747,
        height=4.7747 / 3,
        ticks=TickConfiguration(2, 0.1, 1, 0.05),
        ncol=3,
        bbox=(0, -0.30, 1, 0.1),
        language='serbian',
    ),

    'pde_complex_k_sigma' : RegionConfiguration(
        rust_configuration='pde_complex_k_sigma',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * PDE_W2HRATIO,
        ncol=1,
        bbox=(0, -0.19, 1, 0.1),
    ),

    'pde_complex_k_sigma_thesis' : RegionConfiguration(
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'] * 1.05,
        ticks=TickConfiguration(5, 5, 1, 1),
        rust_configuration='pde_complex_k_sigma',
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        language='serbian',
    ),

    'pde_complex_beta_sigma' : RegionConfiguration(
        rust_configuration='pde_complex_beta_sigma',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * PDE_W2HRATIO,
        ncol=1,
        bbox=(0, -0.19, 1, 0.1),
    ),

    'pde_complex_beta_sigma_thesis' : RegionConfiguration(
        rust_configuration='pde_complex_beta_sigma',
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'],
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        language='serbian',
    ),

    'pde_complex_tau_sigma' : RegionConfiguration(
        rust_configuration='pde_complex_tau_sigma',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * PDE_W2HRATIO,
        ncol=2,
        bbox=(0, -0.19, 1, 0.1),
    ),

    'telegrapher_x_k' : RegionConfiguration(
        rust_configuration='telegrapher_x_k',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * 1.05,
        ncol=2,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'telegrapher_x_k_thesis' : RegionConfiguration(
        rust_configuration='telegrapher_x_k',
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'] * 1.05,
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        language='serbian',
    ),

     'telegrapher_alpha_gamma' : RegionConfiguration(
        rust_configuration='telegrapher_alpha_gamma',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * 1.15,
        ncol=2,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'telegrapher_alpha_gamma_thesis' : RegionConfiguration(
        rust_configuration='telegrapher_alpha_gamma',
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'] * 1.05,
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        language='serbian',
    ),

    'semi_infinite_rod' : RegionConfiguration(
        rust_configuration='semi_infinite_rod',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * ROD_W2HRATIO,
        ncol=2,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'semi_infinite_rod_thesis' : RegionConfiguration(
        rust_configuration='semi_infinite_rod',
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'],
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        language='serbian',
    ),

    'finite_rod' : RegionConfiguration(
        rust_configuration='finite_rod',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * ROD_W2HRATIO,
        ncol=2,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'finite_rod_thesis' : RegionConfiguration(
        rust_configuration='finite_rod',
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'],
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'telegrapher_standard' : RegionConfiguration(
        rust_configuration='telegrapher_standard',
        width=COMMON_WIDTHS['triple_subfigure'],
        height=COMMON_WIDTHS['triple_subfigure'] * ROD_W2HRATIO,
        ncol=2,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'telegrapher_standard_thesis' : RegionConfiguration(
        rust_configuration='telegrapher_standard',
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'],
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        ticks=TickConfiguration(0.05, 0.1, 0.05, 0.1),
    ),

    'pde_complex_instructional' : RegionConfiguration(
        rust_configuration='pde_complex_tau_sigma',
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'] * 1.15,
        ncol=2,
        bbox=(0, -0.19, 1, 0.1),
        language='serbian',
        ticks=TickConfiguration(5, 5, 1, 1),
    ),

    'test_configuration' : RegionConfiguration(
        rust_configuration='test_configuration',
        ncol=3,
        width=COMMON_WIDTHS['single_column'],
        height=COMMON_WIDTHS['single_column'] * W2H_RATIO,
        bbox=(0, -0.17, 1, 0.1),
    ),

    'ln_system1' : RegionConfiguration(
        rust_configuration='ln_system1',
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'],
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        ticks=TickConfiguration(5, 5, 1, 1),
    ),

    'dopid2' : RegionConfiguration(
        rust_configuration='dopid2',
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'],
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        ticks=TickConfiguration(10, 1, 1, 0.5),
    ),

    'dopid3' : RegionConfiguration(
        rust_configuration='dopid3',
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'] * 1.1,
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        ticks=TickConfiguration(1, 1, 1, 1),
    ),

    'ln_system1_negative' : RegionConfiguration(
        rust_configuration='ln_system1_negative',
        width=COMMON_WIDTHS['thesis_standard'],
        height=COMMON_WIDTHS['thesis_standard'] * 1.1,
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        ticks=TickConfiguration(5, 5, 1, 1),
    ),
}