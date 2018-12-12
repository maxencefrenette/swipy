#!/usr/bin/python3

from shared import build, train, plot_many, get_v_function_arg

v_functions = get_v_function_arg()

build()

if v_functions == "small":
    h1 = train("legacy", num_games=30000, alpha=0.0005)
    h2 = train("n_tuple_small", num_games=30000, alpha=0.0005)

    plot_many([("Legacy", h1), ("N-Tuple Small", h2)])
elif v_functions == "medium":
    h = train("n_tuple_medium", num_games=2000000, alpha=0.0025, benchmark_interval=25000)

    plot_many([("N-Tuple Medium", h)])
