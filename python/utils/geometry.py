import numpy as np


def theta2point(origin, theta, angle):
    """Convert given point, direction and magnitude into new point."""
    directional_vector = np.array([np.cos(angle), np.sin(angle)])
    offset = theta * directional_vector
    return origin + offset