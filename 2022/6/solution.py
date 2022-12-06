import collections
import itertools
import math
import os
from pathlib import Path
import re
import sys


INPUT_FILE = Path(__file__).parent / "input.txt"


def main() -> None:
    std_mode = 0

    if not INPUT_FILE.is_file() or std_mode:
        solution(sys.stdin)
    else:
        with open(INPUT_FILE, "r") as f:
           solution(f)

def solution(inp):
    for line in inp:
        for i in range(4, len(line)):
            if len(set(line[i-14:i])) == 14:
                print(i)
                break

    print("no solution found")


if __name__ == "__main__":
    main()
