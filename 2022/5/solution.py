import collections
import itertools
import math
import os
from pathlib import Path
import re
import sys


INPUT_FILE = Path(__file__).parent / "input(1).txt"


def main() -> None:
    std_mode = 0

    if not INPUT_FILE.is_file() or std_mode:
        solution(sys.stdin)
    else:
        with open(INPUT_FILE, "r") as f:
           solution(f)

def solution(inp):
    result = 0
    items = []

    next_line = next(inp)
    n_stacks = (len(next_line) + 1) // 4
    stacks = [[] for _ in range(n_stacks)]

    while next_line != '\n':
        line, next_line = next_line, next(inp)

        for i, stack in enumerate(stacks):
            item = line[4*i + 1]

            if item != ' ':
                stacks[i].append(item)

    for stack in stacks:
        stack.reverse()

    for line in inp:
        _, count, _, from_,_, to_ = line.split(" ")
        count = int(count)
        from_ = int(from_) - 1
        to_ = int(to_) - 1

        items = stacks[from_][-count:]
        stacks[from_] = stacks[from_][:-count]
        stacks[to_].extend(items)

    print(''.join([stack.pop() for stack in stacks]))
    print(result)


if __name__ == "__main__":
    main()
