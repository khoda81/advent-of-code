import itertools
from pathlib import Path

import more_itertools

INPUT_FILE = Path(__file__).parent / "input.txt"


def main() -> None:
    input_path = INPUT_FILE
    if not INPUT_FILE.is_file():
        input_path = 0  # stdin

    # input_path = 0
    with open(input_path, "r") as f:
        f = more_itertools.peekable(f)
        solution(f)


def solution(inp):
    result = 0
    items = [
        
    ]

    for line in inp:
        trees  = list(map(lambda x: [int(x), False], line.strip()))
        items.append(trees)
    
    
    def count(trees, max):
        trees = list(trees)
        answer = 0
        # print(trees)
        # max = -float("inf")
        for t in trees:
            answer += 1
            
            if t[0] >= max[0]:
                break
    
        return answer

    x = len(items)
    y = len(items[0])
    for i, j in itertools.product(range(x), range(y)):
        # print(i, j)
        a = count((items[i][j] for j in range(j+1, y)), items[i][j])
        b = count((items[i][j] for j in range(j-1, -1, -1)), items[i][j])
        c = count((items[i][j] for i in range(i+1, x)), items[i][j])
        d = count((items[i][j] for i in range(i-1, -1, -1)), items[i][j])
        result =  max(result, a*b*c*d)
        # print(a, b, c, d)
        # print(a*b*c*d)
    
    # for i in range(len(items[0])):
    #     tres = [tres[i] for tres in items]
    #     result += count(tres)
    #     result += count(tres[::-1])

    print(result)
    # for tres in items:
    #     print(*tres)


if __name__ == "__main__":
    main()
