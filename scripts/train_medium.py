#!/usr/bin/python3

from shared import build, train, plot_many

build()

h = train("n_tuple_medium", num_games=2000000, alpha=0.0025, benchmark_interval=25000)

plot_many([("N-Tuple Medium", h)])
