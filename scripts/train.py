#!/usr/bin/python3

from shared import build, train, plot_many, get_args, load_json

args = get_args()

build()

if args.v_function == "small":
    if not args.plot_only:
        h1 = train("legacy", num_games=30000, alpha=0.0005)
        h2 = train("n_tuple_small", num_games=30000, alpha=0.0005)
    else:
        h1 = load_json("networks/legacy.training.json")
        h2 = load_json("networks/n_tuple_small.training.json")

    plot_many([("Legacy", h1), ("N-Tuple Small", h2)])
elif args.v_function == "medium":
    if not args.plot_only:
        h = train(
            "n_tuple_medium", num_games=2000000, alpha=0.0025, benchmark_interval=25000
        )
    else:
        h = load_json("networks/n_tuple_medium.training.json")

    plot_many([("N-Tuple Medium", h)])
