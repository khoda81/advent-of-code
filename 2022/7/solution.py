from pathlib import Path

import more_itertools

INPUT_FILE = Path(__file__).parent / "input.txt"


def main() -> None:
    input_path = INPUT_FILE
    if not INPUT_FILE.is_file():
        input_path = 0  # stdin

    with open(input_path, "r") as f:
        f = more_itertools.peekable(f)
        solution(f)


def walk(
    folder,
    parent_value=0,
    file_handler=lambda parent_value, name, size: size,
    folder_handler=lambda parent_value, name, folder: parent_value,
    collector=sum,
):
    results = []
    value = folder_handler(parent_value, folder["."], folder)

    for name, item in folder.items():
        if name in ['.', ".."]:
            continue

        if isinstance(item, int):
            results.append(file_handler(value, name, item))
        else:
            results.append(walk(item, value, file_handler, folder_handler, collector))

    return collector(results)


def print_tree(folder):
    def file_value(indent, name, size):
        print("  " * indent, "-", name, size)
        return size

    def folder_value(indent, name, folder):
        print("  " * indent, "-", name, "(dir)")
        return indent + 1

    return walk(
        folder,
        file_handler=file_value,
        folder_handler=folder_value,
    )


def min_size(folder, lower_bound):
    def collector(results):
        size = sum(size for size, best in results)
        best = min(best for size, best in results)
        if size > lower_bound:
            best = min(best, size)

        return size, best

    return walk(
        folder,
        file_handler=lambda _v, _n, size: (size, float("inf")),
        collector=collector,
    )


def solution(inp: more_itertools.peekable):
    current = root = {}
    root['.'] = "/"  # folder name
    root[".."] = root

    for command in inp:
        if command.startswith("$ ls"):
            for line in inp:
                if line.startswith("$"):
                    inp.prepend(line)
                    break

                size, name = line.strip().split()
                if size == "dir":
                    if name in current:
                        print("duplicate_folder:", name)
                        continue

                    current[name] = new_folder = {}
                    new_folder['.'] = name  # folder name
                    new_folder['..'] = current
                else:
                    current[name] = int(size)
        else:
            path = command.split()[2]
            if path == '/':
                current = root
            else:
                current = current[path]

    total = 70_000_000
    empty = total - walk(root)
    required = 30_000_000 - empty
    print(f"{required=}")

    # print_tree(root)
    print("result:", min_size(root, required))
    # print(sum(i for i in size(root) if i <= 100_000))


if __name__ == "__main__":
    main()
