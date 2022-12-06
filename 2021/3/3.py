inp = [
    "00100",
    "11110",
    "10110",
    "10111",
    "10101",
    "01111",
    "00111",
    "11100",
    "10000",
    "11001",
    "00010",
    "01010"
]

# with open("input3.txt") as f:
#     inp = f.read().splitlines()


def categorize(pos, inp):
    on = []
    off = []

    for item in inp:
        if item[pos] == "1":
            on.append(item)
        else:
            off.append(item)

    return on, off


# oxygen
things = [item for item in inp]
oxygen = []
i = 0
while len(things) > 1:
    on, off = categorize(0, things)
    if len(on) >= len(off):
        things = on


# gamma_bits = [count_bits(i, inp) for i in range(5)]

# bits = ["1" if c >= 0 else "0" for c in counts]
# bits = "".join(bits)
# gamma = int(bits, 2)
# epsilon = ~gamma + 2**len(bits)
# print(gamma * epsilon)
