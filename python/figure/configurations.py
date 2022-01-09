import python.utils.comparison_methods.gu2005 as gu2005


class TickConfiguration:
    __slots__ = ['major_x', 'major_y', 'minor_x', 'minor_y']

    def __init__(self, major_x, major_y, minor_x, minor_y):
        self.major_x = major_x
        self.major_y = major_y
        self.minor_x = minor_x
        self.minor_y = minor_y


class FigureConfiguration:
    __slots__ = ['ticks', 'custom_func', 'ncol', 'bbox']

    def __init__(self, ticks, custom_func, ncol, bbox):
        self.ticks = ticks
        self.custom_func = custom_func
        self.ncol = ncol
        self.bbox = bbox


class LineConfiguration(FigureConfiguration):
    __slots__ = ['ratios']

    def __init__(self, ticks, custom_func, ncol, bbox, ratios):
        super().__init__(ticks, custom_func, ncol, bbox)
        self.ratios = ratios


def retarded1_custom_func(ax):
    gu2005.add_gu2005_example1(ax)


CONFIGURATIONS = {
    'retarded1' : LineConfiguration(
        ticks=TickConfiguration(1, 1, 0.25, 0.25),
        custom_func=retarded1_custom_func,
        ncol=3,
        bbox=(0, -0.17, 1, 0.1),
        ratios=[1, 2, 5, 5, 8],
    ),
}