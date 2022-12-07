import collections
import itertools
import math
import os
from pathlib import Path
import re
import sys


INPUT_PATH = Path(__file__).parent / "input.txt"


def main() -> None:
    input_path = INPUT_PATH
    if not input_path.is_file():
        input_path = 0 # open(0) will open stdin as a file

    with open(input_path, "r") as f:
        solution(f)

def solution(inp):
    root = {}
    root['.']= '/'
    root[".."] = root
    
    current = root

    def get_path(folder):
        if folder == root:
            return "/"

        path = []
        while folder != root:
            path.append(folder['.'])
            folder = folder['..']

        return "/".join(reversed(path))

    running = True
    command = next(inp)
    while running:
        if command.startswith("$ ls"):
            for line in inp:
                if line.startswith("$"):
                    command = line
                    break

                a, b = line.strip().split()
                if a.isnumeric():
                    current[b] = int(a)
                else:
                    if b not in current:
                        current[b] = new_folder = {}
                        new_folder['.'] = b
                        new_folder['..'] = current
            else:
                running = False
        else:
            path = command.split()[2]
            if path == '/':
                current = root
            else:
                current = current[path]

            command = next(inp)

    def size(folder):
        result_size = 0
        for k, v in folder.items():
            if k == '.' or k == '..':
                continue

            if isinstance(v, int):
                result_size += v
            else:
                result_size += size(v)

        return result_size
    
    def print_tree(folder, indent=0):
        print("  " * indent, "-", folder["."], "(dir)")
        
        for k, v in folder.items():
            if k in ['.', ".."]:
                continue
            
            if isinstance(v, int):
                print("  " * indent, "  -", k, v)
            else:        
                print_tree(v, indent + 1)

    total = 70_000_000
    empty = total - size(root)
    required = 30_000_000 - empty
    print(required)

    result = float("inf")
    def find_best(folder):
        nonlocal result
        result_size = 0
        for k, v in folder.items():
            if k == '.' or k == '..':
                continue

            if isinstance(v, int):
                result_size += v
            else:
                result_size += find_best(v)

        if result_size > required:
            result = min(result, result_size)

        return result_size
    
    # print_tree(root)
    find_best(root)
    print("result:", result)
    # print(sum(i for i in size(root) if i <= 100_000))


if __name__ == "__main__":
    main()
