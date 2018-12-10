#!/usr/bin/python3

import json
import shared

def load_json(file_name):
    with open(file_name) as file:
        return json.load(file)

h1 = load_json("networks/legacy.training.json")
h2 = load_json("networks/n_tuple_small.training.json")

shared.plot_many([("Legacy", h1), ("N-Tuple Small", h2)])
