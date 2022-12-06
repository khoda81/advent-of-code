import sys


def main() -> None:
    result = 0

    for bag1 in sys.stdin:
        bag2 = next(sys.stdin).strip()
        bag3 = next(sys.stdin).strip()
        item = (set(bag1) & set(bag2) & set(bag3)).pop()
        result += (
            (ord(item) - ord("a") + 1)
            if item.islower()
            else (ord(item) - ord("A") + 27)
        )

    print(result)


if __name__ == "__main__":
    main()
