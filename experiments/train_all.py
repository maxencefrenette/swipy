#!/usr/bin/python3

import json
import os
from subprocess import run, Popen, PIPE
import shared

run(["cargo", "build", "--release"])

executable = ["./target/release/swipy-cli"]


def train(v_function):
    process = Popen(
        executable + ["train", "30000", "-z", "--format", "json"],
        stdout=PIPE,
        encoding="UTF-8",
    )

    games = []
    training_scores = []
    test_scores = []

    for line in process.stdout:
        message = json.loads(line)

        if "game" in message:
            games.append(message["game"])
            training_scores.append(message["training_score"])
            test_scores.append(message["test_score"])

            print(f"Game {message['game']}, Average Score: {message['test_score']}")
        else:
            if os.path.isfile(f"networks/{v_function}.json"):
                os.replace(
                    f"networks/{v_function}.json", f"networks/{v_function}.backup.json"
                )

            with open(f"networks/{v_function}.json", "w") as file:
                file.write(line)

    history = {
        "games": games,
        "training_scores": training_scores,
        "test_scores": test_scores,
    }

    with open(f"networks/{v_function}.training.json", "w") as file:
        json.dump(history, file)

    return history


h1 = train("legacy")
h2 = train("n_tuple_small")

shared.plot_many([("Legacy", h1), ("N-Tuple Small", h2)])
