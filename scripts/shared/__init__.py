from subprocess import run, Popen, PIPE
from .plotting import *
from .train import *

executable = ["./target/release/swipy-cli"]

def build():
    run(["cargo", "build", "--release"])
