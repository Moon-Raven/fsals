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


def read_data(path):
    data = None

    with open(path, 'r') as read_file:
        data = json.load(read_file, object_hook=lambda d: SimpleNamespace(**d))

    if data == None:
        raise Exception(f'Error reading nu results from file')

    return data


def instructional_line_sufficient(args):
    data = read_data(f'output/data/line/pde_complex_tau_sigma_instructional.data')
    breakpoint()


def main(args):
    logger.info(f'Running custom script {args.customscript}!')

    if args.customscript == 'instructional_line_sufficient':
        instructional_line_sufficient(args)