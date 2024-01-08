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
        
def display_grid(grid):
    print()
    for i, row in enumerate(grid):
        print(f"{i:3d}", ''.join(
            "#" if cell == 1 else 
            "o" if cell == 2 else
            "?" if cell else
            "." for cell in row
            ))

    # input()

def solution(inp):
    result = 0
    paths = []
    min_x = INF
    min_y = 0
    max_x = max_y = -INF

    for path_string in inp:
        path = []
        for point in path_string.split(" -> "):
            x, y = map(int, point.split(","))
            min_x = min(min_x, x)
            min_y = min(min_y, y)
            max_x = max(max_x, x)
            max_y = max(max_y, y)

            path.append((x, y))

        paths.append(path)

    print(min_x, max_x)
    print(min_y, max_y)
    max_y += 2
    min_x = min(min_x, 500 - max_y)
    max_x = max(max_x, 500 + max_y)
    
    w = max_x - min_x + 1
    h = max_y - min_y + 1
    print(w, h)
    grid = np.zeros((h, w), dtype=int)
    grid[-1] = 1
    for path in paths:
        path = iter(path)
        x, y = next(path)
        for nx, ny in path:
            start_y, end_y = sorted([y-min_y, ny-min_y])
            start_x, end_x = sorted([x-min_x, nx-min_x])
            grid[start_y: end_y+1, start_x: end_x+1] = 1
            x, y = nx, ny

    source = np.array([0, 500-min_x])

    down = np.array([1, 0])
    right = np.array([0, 1])
    display_grid(grid)
    while grid[tuple(source)] == 0:
        sand = source.copy()

        while h - 1 > sand[0] >= 0 and w > sand[1] >= 0:
            if not grid[tuple(sand + down)]:
                sand += down
            elif not grid[tuple(sand + down - right)]:
                sand += down - right
            elif not grid[tuple(sand + down + right)]:
                sand += down + right
            else:
                grid[tuple(sand)] = 2
                break
        else:
            break

        result += 1

    display_grid(grid)
    print(result)


if __name__ == "__main__":
    main()
