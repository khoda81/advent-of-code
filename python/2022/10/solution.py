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
    x = 1
    clock = 0
    screen = [["." for _ in range(40)] for _ in range(6)]
    
    def update():
        nonlocal clock, result, screen
        clock += 1
        r, c = divmod(clock, 40)
        if -2 < x - c < 2:
            screen[r][c] = "#"

        if c == 20:
            result += x * clock
            print(x, clock)

    update()
    for line in inp:
        match line.split():
            case "addx", value:
                x += int(value)
                update()
            case "noop",:
                pass
            case [*args]:
                print("unknown instruction:", *args)

        update()
    print(result)
    for row in screen:
        print(''.join(row))


if __name__ == "__main__":
    main()
