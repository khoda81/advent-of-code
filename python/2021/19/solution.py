import sys
from pathlib import Path

INPUT_FILE = Path(__file__).parent / "input.txt"


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

    for line in inp:
        print(line)

    print(result)


if __name__ == "__main__":
    main()
