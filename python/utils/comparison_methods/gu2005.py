import logging

import numpy as np


def add_SCS_curves(ax, curves, color='black', linestyle='-'):
    for curve in curves:
        ax.plot(curve[0,:], curve[1,:], color=color, linestyle=linestyle)


def add_gu2005_example1(ax):
    def a1(s): return 2/s
    def a2(s): return 1/np.power(s,2)
    SCS_curves = get_SCS_curves(a1, a2)
    add_SCS_curves(ax, SCS_curves, color='black', linestyle='--')


def get_OMEGA(a1, a2, N=100000):
    w = np.linspace(1e-10, 10, N)
    eq32 = np.abs(a1(1j*w)) + np.abs(a2(1j*w))
    eq33 = np.abs(np.abs(a1(1j*w)) - np.abs(a2(1j*w)))
    OMEGA_condition = np.logical_and((eq32 >= 1), (eq33 <= 1))
    intervals = []
    current_interval_valid = False
    current_interval = None

    for i in range(N):
        if current_interval_valid:
            if OMEGA_condition[i]:
                pass
            else:
                current_interval[1] = w[i-1]
                if current_interval[0] == current_interval[1]:
                    logging.warning(f'Interval start and end points are the' \
                                     ' same for w={w[i-1]}!')
                intervals.append(current_interval)
                current_interval = None
                current_interval_valid = False
        else:
            if OMEGA_condition[i]:
                current_interval = [w[i], None]
                current_interval_valid = True
            else:
                pass
    return intervals


def get_thetas(a1_func, a2_func, w):
    a1 = np.abs(a1_func(1j*w))
    a2 = np.abs(a2_func(1j*w))
    theta1 = np.arccos((1 + np.power(a1,2) - np.power(a2,2)) / (2*a1))
    theta2 = np.arccos((1 + np.power(a2,2) - np.power(a1,2)) / (2*a2))
    return  theta1, theta2


def find_first_true(start_value, f):
    first = True
    x = start_value
    while True:
        res = f(x)
        if res:
            if first:
                raise Exception('Found up at first attempt!')
            else:
                return x
        else:
            first = False
            x += 1


def get_us(a1, theta1_array, w_array):
    INITIAL_U = -3
    N = len(w_array)
    up = np.empty(w_array.shape)
    um = np.empty(w_array.shape)
    for i in range(N):
        w = w_array[i]
        a1_angle = np.angle(a1(1j*w))
        theta1 = theta1_array[i]
        def fp(u): return (((a1_angle + (2*u-1)*np.pi + theta1) / w) > 0)
        up[i] = find_first_true(INITIAL_U, fp)
        def fm(u): return (((a1_angle + (2*u-1)*np.pi - theta1) / w) > 0)
        um[i] = find_first_true(INITIAL_U, fm)
    return up, um


def get_vs(a2, theta2_array, w_array):
    INITIAL_U = -3
    N = len(w_array)
    vp = np.empty(w_array.shape)
    vm = np.empty(w_array.shape)
    for i in range(N):
        w = w_array[i]
        a2_angle = np.angle(a2(1j*w))
        theta2 = theta2_array[i]
        def fp(u): return (((a2_angle + (2*u-1)*np.pi - theta2) / w) > 0)
        vp[i] = find_first_true(INITIAL_U, fp)
        def fm(u): return (((a2_angle + (2*u-1)*np.pi + theta2) / w) > 0)
        vm[i] = find_first_true(INITIAL_U, fm)
    return vp, vm


def generate_SCS_curve(a1_angle,a2_angle,theta1,theta2,u,v,w):
        tau1_p_array = (a1_angle + (2*u-1)*np.pi + theta1) / w
        tau2_p_array = (a2_angle + (2*v-1)*np.pi - theta2) / w
        tau1_m_array = (a1_angle + (2*u-1)*np.pi - theta1) / w
        tau2_m_array = (a2_angle + (2*v-1)*np.pi + theta2) / w
        p_curve = np.vstack((tau1_p_array, tau2_p_array))
        m_curve = np.vstack((tau1_m_array, tau2_m_array))
        return p_curve, m_curve


def list2curve(point_list):
    curve = np.vstack(point_list).T
    return curve


def filter_curve(curve):
    OFF, ON = 0, 1
    n = curve.shape[1]
    state = OFF
    resulting_curves, current_curve = [], []
    for i in range(n):
        if state == OFF:
            if (curve[:,i] > 0).all():
                state = ON
                current_curve.append(curve[:,i])
            else:
                pass
        elif state == ON:
            if (curve[:,i] > 0).all():
                current_curve.append(curve[:,i])
            else:
                state = OFF
                new_curve = list2curve(current_curve)
                resulting_curves.append(new_curve.copy())
                current_curve = []
        else:
            raise ValueError(f'Unknown state {state}')

    if state == ON:
        new_curve = list2curve(current_curve)
        resulting_curves.append(new_curve)

    return resulting_curves


def OMEGA2SCS_curves(a1, a2, OMEGA_intervals, N=1000, DRAW_DEPTH=10):
    result_curves = []
    interval_counter = 0

    for interval_counter in range(len(OMEGA_intervals)):
        interval = OMEGA_intervals[interval_counter]
        w1, w2 = interval[0], interval[1]
        omega_array = np.linspace(w1, w2, N)
        theta1, theta2 = get_thetas(a1, a2, omega_array)

        up, um = get_us(a1, theta1, omega_array)
        vp, vm = get_vs(a2, theta2, omega_array)
        u_min = np.min(np.concatenate((up,um)))
        v_min = np.min(np.concatenate((vp,vm)))

        w = omega_array
        a1_angle = np.angle(a1(1j*w))
        a2_angle = np.angle(a2(1j*w))

        for i in range(DRAW_DEPTH):
            for k in range(DRAW_DEPTH):
                curve_p, curve_m = generate_SCS_curve(a1_angle,a2_angle,theta1,
                                                      theta2,u_min+i,v_min+k,w)
                new_p_curves = filter_curve(curve_p)
                new_m_curves = filter_curve(curve_m)
                result_curves += new_p_curves
                result_curves += new_m_curves

    return result_curves


def get_SCS_curves(a1, a2):
    intervals = get_OMEGA(a1, a2)
    curves = OMEGA2SCS_curves(a1, a2, intervals)
    return curves