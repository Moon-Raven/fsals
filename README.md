## Introduction

Framework for Stability Analysis of Linear Systems (FSALS)
analyzes the stability of dynamical systems.
The framework is applicable to linear systems with irrational transfer functions.
Most common examples of such dynamical systems include time-delay systems,
fractional systems and distributed-parameter systems.

The framework implements theoretical methods proposed in
[this FCAA paper](https://doi.org/10.1007/s13540-022-00126-z) and
[this Automatica paper](https://doi.org/10.1016/j.automatica.2023.111220).
The methods are based on iterative application of
[Rouché's theorem](https://en.wikipedia.org/wiki/Rouch%C3%A9%27s_theorem),
[Hölder's inequality](https://en.wikipedia.org/wiki/H%C3%B6lder%27s_inequality)
and the [Fundamental theorem of calculus](https://en.wikipedia.org/wiki/Fundamental_theorem_of_calculus).
The framework operates on systems described by characteristic functions -
a concept similar to [transfer functions](https://en.wikipedia.org/wiki/Transfer_function)
and [characteristic polynomials](https://en.wikipedia.org/wiki/Characteristic_polynomial).


## Usage

The user can invoke commands by starting the `main.py` script,
and providing appropriate parameters.
The following commands are available:

* `data`
: Performs numeric calculations, and stores the results in data files.
* `figure`
: Reads results from data files, and visualizes them.
* `nu`
: Performs spot-check stability analysis on a point-grid using
[Cauchy's argument principle](https://en.wikipedia.org/wiki/Argument_principle).

The most important options for running the script are:
* `--configuration`: Specifies the configuration to be run.
* `--algorithm`: Specifies the algorithm to be used (`line` or `region`).

The full set of options can be obtained by running
```
python main.py --help
```

Several usage examples:

```
python main.py --configuration distributed_delay1 --algorithm line data
```

```
python main.py --configuration finite_rod --algorithm region data
```

```
python main.py --configuration semi_infinite_rod --algorithm region figure
```

```
python main.py --configuration telegrapher_standard nu
```

Common usage scenario:

1. Sketch stability equivalence regions for a system using `nu`:
    ```
    python main.py --configuration telegrapher_standard nu
    ```
1. Obtain numerical results using `data`:
    ```
    python main.py --configuration telegrapher_standard --algorithm region data
    ```
1. Plot the results using `figure`:
    ```
    python main.py --configuration telegrapher_standard --algorithm region figure
    ```


## Architecture

The CLI is implemented in Python, which acts as a front-end for accepting user commands.
Each command (`nu`, `data`, `figure`) is then implemented via a different "engine".
The engines for `nu` and `data` are implemented in Rust,
while the engine for `figure` is implemented in Python.

Each dynamical system (simply denoted `system` in the project)
is represented by a separate Rust source file in `rust/src/systems/`.
The file contains numerical functions which specify the system behavior,
as well as functions necessary to implement the algorithms.
These functions must be obtained analytically, according to
the [theoretical background](https://doi.org/10.1007/s13540-022-00126-z)
of the methods.

The algorithms (`nu`, `line` and `region`) are stored in separate Rust modules
(`rust/src/nu`, `rust/src/data/line` and `rust/src/data/region`).
The algorithms can be invoked on a dynamical
systems by using different parameters, such as tolerance, precision and domain limits.
Each such set of parameters is called a `configuration`, and is applicable to a given `system`.
The configurations are stored in seperate Rust source files for each algorithm
(e.g. `rust/src/data/line/configurations.rs`).

The engine for plotting figures is implemented in Python,
and based on [`matplotlib`](https://matplotlib.org/).
The figures can be parametrized in various ways,
which each set of parameters forming a figure `configuration`.
These configurations are stored in `python/figure/configurations.py`.