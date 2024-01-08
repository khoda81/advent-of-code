# forward 5
# down 5
# forward 8
# up 3
# down 8
# forward 2

inp = [
    ("forward", "5"),
    ("down", "5"),
    ("forward", "8"),
    ("up", "3"),
    ("down", "8"),
    ("forward", "2"),
]

with open("input2.txt") as f:
    inp = [line.split() for line in f.readlines()]

pos = [0, 0]
aim = 0
# down X increases your aim by X units.
# up X decreases your aim by X units.
# forward X does two things:
#   It increases your horizontal position by X units.
#   It increases your depth by your aim multiplied by X.

for d, n in inp:
    n = int(n)
    match d:
        case "up":
            aim += n
        case "down":
            aim -= n
        case "forward":
            pos[0] += n
            pos[1] += aim * n

print(pos[0] * pos[1])
