with open("input1.txt") as f:
    lines = f.readlines()
    inp = [int(x) for x in lines]

count = 0
s = inp[0] + inp[1] + inp[2]
for i in range(3, len(inp)):
    new_s = s + inp[i] - inp[i-3]
    count += new_s > s
    s = new_s

print(count)
