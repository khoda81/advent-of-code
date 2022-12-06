import sys


def main() -> None:
    result = 0

    for line in sys.stdin:
        first, second = line.split(',')
        a1,b1 = map(int, first.split("-"))
        a2,b2 = map(int, second.split("-"))
        # print(a1, a2, b1, b2)

        a = range(a1, b1 + 1)
        b = range(a2, b2 + 1)

        if a1 in b or b1 in b or a2 in a:
            result += 1

    print(result)


if __name__ == "__main__":
    main()
