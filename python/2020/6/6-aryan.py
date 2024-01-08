s = 0
flagStart = True

for line in a.split('\n'):
    if (line == ""):
        s += len(set(tmp))
        flagStart = True
    else:
        if flagStart:
            tmp = set(line)
            flagStart = False
        else:
            tmp.intersection_update(set(line))

print(s)
