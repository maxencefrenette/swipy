#!/usr/bin/python3

import json
from shared import load_json, plot_many

h1 = load_json("networks/legacy.training.json")
h2 = load_json("networks/n_tuple_small.training.json")

plot_many([("Legacy", h1), ("N-Tuple Small", h2)])
