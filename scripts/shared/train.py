import json
import os
from subprocess import run, Popen, PIPE
from .__init__ import executable


def train(v_function, num_games, alpha, benchmark_interval=5000):
    process = Popen(
        executable
        + [
            "train",
            f"{num_games}",
            "-z",
            "--v_function",
            f"{v_function}",
            "--alpha",
            f"{alpha}",
            f"--benchmark-interval={benchmark_interval}",
            "--format",
            "json",
        ],
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
            if os.path.isfile(f"networks/{v_function}.json") or os.path.isfile(
                f"networks/{v_function}.training.json"
            ):
                os.mkdir("networks/backup")

            move_if_exists(
                f"networks/{v_function}.json", f"networks/backup/{v_function}.json"
            )
            move_if_exists(
                f"networks/{v_function}.training.json",
                f"networks/backup/{v_function}.training.json",
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


def move_if_exists(source, dest):
    if os.path.isfile(source):
        os.replace(source, dest)
