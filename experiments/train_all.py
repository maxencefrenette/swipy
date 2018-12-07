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

    return games, training_scores, test_scores


games1, training1, test1 = train("legacy")
games2, training2, test2 = train("n_tuple_small")

fig, (ax1, ax2) = plt.subplots(nrows=2, sharex=True)

ax1.set_title("Legacy V-Function")
ax1.plot(games1, training1, "C1")
ax1.twinx().plot(games1, test1, "C2")

ax2.set_title("N-Tuple Small V-Function")
ax2.plot(games2, training2, "C1")
ax2.twinx().plot(games2, test2, "C2")

train_line = mlines.Line2D([], [], color='C1', label="Training")
test_line = mlines.Line2D([], [], color='C2', label="Testing")
ax2.legend(handles=[train_line, test_line], bbox_to_anchor=(0.5, -.11), loc="upper center", shadow=True, ncol=2)

fig.tight_layout()

plt.show()
