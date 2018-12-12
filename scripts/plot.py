#!/usr/bin/python3

from shared import load_json, plot_many, get_v_function_arg

v_functions = get_v_function_arg()

if v_functions == "small":
    h1 = load_json("networks/legacy.training.json")
    h2 = load_json("networks/n_tuple_small.training.json")

    plot_many([("Legacy", h1), ("N-Tuple Small", h2)])
elif v_functions == "medium":
    h = load_json("networks/n_tuple_medium.training.json")

    plot_many([("N-Tuple Medium", h)])
