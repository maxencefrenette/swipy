from subprocess import run, Popen, PIPE
from .plotting import *
from .train import *

executable = ["./target/release/swipy-cli"]

def build():
    run(["cargo", "build", "--release"])

def load_json(file_name):
    with open(file_name) as file:
        return json.load(file)
