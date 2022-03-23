#!/bin/bash

LOGNAME=script_progress.txt
rm -f $LOGNAME

SYSTEMS=()
SYSTEMS+=(retarded1)
SYSTEMS+=(distributed_delay1)
SYSTEMS+=(semi_infinite_rod)
SYSTEMS+=(finite_rod)
SYSTEMS+=(pde_complex_k_sigma)
SYSTEMS+=(pde_complex_tau_sigma)
SYSTEMS+=(pde_complex_beta_sigma)
SYSTEMS+=(telegrapher_x_k)
SYSTEMS+=(telegrapher_alpha_gamma)

ALGORITHMS=(line region)

for system in ${SYSTEMS[@]}; do
    for algo in ${ALGORITHMS[@]}; do
        echo $system $algo >> $LOGNAME
        cargo run --release -- -l info -a $algo -s $system data
    done
done