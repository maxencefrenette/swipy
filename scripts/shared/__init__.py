from argparse import ArgumentParser
from subprocess import run, Popen, PIPE
from .plotting import *
from .train import *

executable = ["./target/release/swipy-cli"]


def build():
    run(["cargo", "build", "--release"])


def load_json(file_name):
    with open(file_name) as file:
        return json.load(file)


def get_v_function_arg():
    parser = ArgumentParser()
    parser.add_argument("v_function")
    args = parser.parse_args()
    return args.v_function
