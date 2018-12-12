#!/usr/bin/python3

from shared import build, train, plot_many

build()

h = train("n_tuple_medium", num_games=1000000, alpha=0.0005, benchmark_interval=25000)

plot_many([("N-Tuple Medium", h)])
