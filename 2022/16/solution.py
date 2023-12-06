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


def solution(inp):
    result = 0
    items = []

    for line in inp:
        print(line)
        result += 1

    print(result)


if __name__ == "__main__":
    main()
