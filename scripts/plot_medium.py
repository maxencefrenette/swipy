#!/usr/bin/python3

import json
from shared import load_json, plot_many

h = load_json("networks/n_tuple_medium.training.json")

plot_many([("N-Tuple Medium", h)])
