import collections
import itertools
import math
import re
import sys


def main() -> None:
    result = 0
    items = []
    stacks = [[] for i in range(9)]
    for i in range(8):
        line = next(sys.stdin)
        line = f"{line:<20}"
        for i in range(9):
            item = line[i*4+1]
            if item != ' ':
                stacks[i].append(item)

    next(sys.stdin)
    next(sys.stdin)
    for stack in stacks:
        stack.reverse()

    # print(stacks)
    for line in sys.stdin:
        _, count, _, from_,_, to_ = line.split(" ")
        from_ = int(from_) - 1
        to_ = int(to_) - 1
        items = []
        for i in range(int(count)):
            if not stacks[from_]:
                break

            items.append(stacks[from_].pop())
        while items:
            stacks[to_].append(items.pop())
        # result += 1
            # # print(stacks)
    
            # stacks[to_].append(stacks[from_].pop())
    
    # print(stacks)                 
    print(''.join([stack.pop() for stack in stacks]))
    print(result)


if __name__ == "__main__":
    main()
