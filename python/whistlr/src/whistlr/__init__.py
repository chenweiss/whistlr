import sys

__version__ = "0.0.1"

def main():
    if "--version" in sys.argv:
        print(f"whistlr {__version__}")
    else:
        print("usage: whistlr [--version]")
