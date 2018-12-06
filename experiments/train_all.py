#!/usr/bin/python3

import json
import os
from subprocess import run, Popen, PIPE

run(["cargo", "build", "--release"])

executable = ["./target/release/swipy-cli"]

def train(v_function):
    process = Popen(executable + ["train", "50000", "-z", "--format", "json"], stdout=PIPE, encoding="UTF-8")
    for line in process.stdout:
        message = json.loads(line)

        if "game" in message:
            print(f"Game {message['game']}, Average Score: {message['score']}")
        else:
            if os.path.isfile(f"networks/{v_function}.json"):
                os.replace(f"networks/{v_function}.json", f"networks/{v_function}.backup.json")

            with open(f"networks/{v_function}.json", "w") as file:
                file.write(line)

train("legacy")
train("n_tuple_small")
