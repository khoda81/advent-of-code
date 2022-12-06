from string import ascii_lowercase

total_yes = 0
letters = set(ascii_lowercase)

with open("6.txt", "r") as f:
    for line in f.readlines():
        if line == '\n':
            total_yes += len(letters)
            letters = set(ascii_lowercase)
            continue

        letters.intersection_update(line)

total_yes += len(letters)
print(total_yes)
