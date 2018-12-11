#!/usr/bin/python3

from shared import build, train, plot_many

build()

h1 = train("legacy", num_games=30000, alpha=0.0005)
h2 = train("n_tuple_small", num_games=30000, alpha=0.0005)

plot_many([("Legacy", h1), ("N-Tuple Small", h2)])
