import sys


def main():
    total = 0
    for line in sys.stdin:
        a, b = line.split()
        a = ord(a) - ord('A')
        b = (ord(b) - ord('X') - 1) % 3

        outcome = (b + a) % 3
        outcome_type = (b - 1) % 3
        score = outcome + outcome_type * 3 + 1

        print(score, outcome, a, b)
        total += score

    print(total)


if __name__ == "__main__":
    main()
