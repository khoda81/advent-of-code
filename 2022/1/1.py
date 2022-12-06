import sys

import requests


def main():
    stream = sys.stdin
    stream = requests.get("https://adventofcode.com/2022/day/1/input", stream=True).iter_lines()

    inventories = []
    current_inventory = []

    for line in stream:
        line = line.strip()
        if line == "":
            inventories.append(current_inventory)
            current_inventory = []
            continue

        cal = int(line)
        current_inventory.append(cal)

    if current_inventory:
        current_inventory.append(cal)

    print(inventories)
    inventories = sorted(map(sum, inventories))
    print(inventories[-3:])
    print(sum(inventories[-3:]))


if __name__ == "__main__":
    main()