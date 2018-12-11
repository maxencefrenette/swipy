import json
import os
from subprocess import run, Popen, PIPE
from .__init__ import executable


def train(v_function, num_games, alpha):
    process = Popen(
        executable
        + ["train", f"{num_games}", "-z", "--alpha", f"{alpha}", "--format", "json"],
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
