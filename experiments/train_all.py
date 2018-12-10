#!/usr/bin/python3

import json
import os
from subprocess import run, Popen, PIPE
import matplotlib.pyplot as plt
import matplotlib.lines as mlines

run(["cargo", "build", "--release"])

executable = ["./target/release/swipy-cli"]


def train(v_function):
    process = Popen(executable + ["train", "30000", "-z",
                                  "--format", "json"], stdout=PIPE, encoding="UTF-8")

    games = []
    training_scores = []
    test_scores = []

    for line in process.stdout:
        message = json.loads(line)

        if "game" in message:
            games.append(message["game"])
            training_scores.append(message["training_score"])
            test_scores.append(message["test_score"])

            print(
                f"Game {message['game']}, Average Score: {message['test_score']}")
        else:
            if os.path.isfile(f"networks/{v_function}.json"):
                os.replace(f"networks/{v_function}.json",
                           f"networks/{v_function}.backup.json")

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

fig, (ax1, ax2) = plt.subplots(nrows=2, sharex=True)

ax1.set_title("Legacy V-Function")
ax1.plot(h1["games"], h1["training_scores"], "C1")
ax1.twinx().plot(h1["games"], h1["test_scores"], "C2")

ax2.set_title("N-Tuple Small V-Function")
ax2.plot(h2["games"], h2["training_scores"], "C1")
ax2.twinx().plot(h2["games"], h2["test_scores"], "C2")

train_line = mlines.Line2D([], [], color='C1', label="Training")
test_line = mlines.Line2D([], [], color='C2', label="Testing")
ax2.legend(handles=[train_line, test_line], bbox_to_anchor=(
    0.5, -.11), loc="upper center", shadow=True, ncol=2)

fig.tight_layout()

plt.show()
