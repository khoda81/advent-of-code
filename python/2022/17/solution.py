from itertools import cycle
from pathlib import Path

import more_itertools
import numpy as np

INPUT_FILE = Path(__file__).parent / "input.txt"
INF = float("inf")


def main() -> None:
    input_path = INPUT_FILE
    if not INPUT_FILE.is_file():
        input_path = 0  # stdin

    with open(input_path, "r") as f:
        f = more_itertools.peekable(f)
        solution(f)

def display_tower(grid):
    print()
    for i, row in enumerate(reversed(grid)):
        row = bin(row)[2:].zfill(7).replace("1", "#").replace("0", ".")
        print(f"{i:3d} {row}")


def solution(inp):
    pieces = [
        np.array([30, 0, 0, 0]), # -
        np.array([8, 28, 8, 0]), # +
        np.array([28, 4, 4, 0]), # reversed L
        np.array([16, 16, 16, 16]), # |
        np.array([24, 24, 0, 0]), # square
    ]# display_grid(grid[max_h+3::-1])# display_grid(grid[max_h+3::-1])

    poofs = cycle(next(inp))
    pieces = cycle(pieces)
    piece = next(pieces).copy()
    offset = -1
    n = 50000
    
    tower = [127, 0, 0, 0, 0]
    offsets = []
    for poof in poofs:
        if n <= 0:
            break

        old_piece = piece.copy()
        if poof == "<":
            if not (piece & 64).any():
                piece <<= 1
        else:
            if not (piece & 1).any():
                piece >>= 1

        # check for collision
        if any(
            piece[i] & tower[i + offset] 
            for i in range(4)
            if i + offset < 0
        ):
            piece = old_piece

        offset -= 1
        # check for collision:
        if any(
            piece[i] & tower[i + offset] 
            for i in range(4)
            if i + offset < 0
        ):
            # place piece
            offset += len(tower) + 1
            tower[offset: offset+4] |= piece
            old_tower_height = len(tower)
            for i in tower[-4:]:
                if i:
                    tower.append(0)
            
            offsets.append(len(tower) - old_tower_height)

            # display_tower(tower)
            piece = next(pieces).copy()
            offset = -1
            n -= 1
            
            # lo = 1
            # hi = len(tower) - 3
            # mid = (lo + hi) // 2
            # if tower[lo:mid] == tower[mid:hi]:
            #     print("cycle_found: ", mid-lo)
            #     break

    # display_tower(tower)

    # display_grid(grid[max_h+3::-1])
    # remove last zeros
    while tower[-1] == 0:
        tower.pop()

    print(offsets)
    print(len(tower), sum(offsets))


if __name__ == "__main__":
    main()
