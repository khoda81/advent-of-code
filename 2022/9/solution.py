from pathlib import Path

import more_itertools

INPUT_FILE = Path(__file__).parent / "input.txt"
INF = float("inf")


def main() -> None:
    input_path = INPUT_FILE
    if not INPUT_FILE.is_file():
        input_path = 0  # stdin

    with open(input_path, "r") as f:
        f = more_itertools.peekable(f)
        solution(f)


class pos(tuple):
    def __new__(cls, *args):
        return super().__new__(cls, args)

    def __add__(self, other):
        return pos(*(x + y for x, y in zip(self, other)))

    def __sub__(self, other):
        return pos(*(x - y for x, y in zip(self, other)))

    def __radd__(self, other):
        return self + other

    def __rsub__(self, other):
        return pos(*(y - x for x, y in zip(self, other)))
    
    def __neg__(self):
        return pos(*(-x for x in self))


def solution(inp):
    # index to position:
    # 0 1 2
    # 3 4 5
    # 6 7 8

    idx_to_pos = [divmod(idx, 3) - pos(1, 1) for idx in range(9)]
    name_to_pos = {"U": pos(-1, 0), "L": pos(0, -1), "R": pos(0, 1), "D": pos(1, 0)}

    rope = [pos(0, 0)] * 10
    visited = {rope[-1]}
    def new_tail(head, tail):
        r, c = tail - head + pos(2, 2)
        move_table = [
            [8, 8, 7, 6, 6],
            [8, 4, 4, 4, 6],
            [5, 4, 4, 4, 3],
            [2, 4, 4, 4, 0],
            [2, 2, 1, 0, 0],
        ]

        return tail + idx_to_pos[move_table[r][c]]

    for line in inp:
        d, count = line.split()
        for _ in range(int(count)):
            rope[0] += name_to_pos[d]
            for i in range(len(rope) - 1):
                rope[i+1] = new_tail(rope[i], rope[i+1])

            visited.add(rope[-1])

    # grid = [["."]*10 for))

    # grid = [["."]*10 for i in range(10)]
    # for r, c in visited:
    #     grid[-r][c] = "#"
    # for row in reversed(grid):
    #     print("".join(row))
    print(len(visited))


if __name__ == "__main__":
    main()
