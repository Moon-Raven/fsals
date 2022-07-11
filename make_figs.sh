#!/bin/bash

LOGNAME=script_progress.txt
rm -f $LOGNAME

CONFIGURATIONS=()
CONFIGURATIONS+=(retarded1)
CONFIGURATIONS+=(distributed_delay1)
CONFIGURATIONS+=(semi_infinite_rod)
CONFIGURATIONS+=(finite_rod)
CONFIGURATIONS+=(pde_complex_k_sigma)
CONFIGURATIONS+=(pde_complex_tau_sigma)
CONFIGURATIONS+=(pde_complex_beta_sigma)
CONFIGURATIONS+=(telegrapher_x_k)
CONFIGURATIONS+=(telegrapher_alpha_gamma)
CONFIGURATIONS+=(telegrapher_standard)

ALGORITHMS=()
ALGORITHMS+=(line)
ALGORITHMS+=(region)

for configuration in ${CONFIGURATIONS[@]}; do
    for algo in ${ALGORITHMS[@]}; do
        echo $configuration $algo >> $LOGNAME
        python3 main.py -l info -a $algo -c $configuration figure
    done
done