from pathlib import Path
from pprint import pprint

PATH = Path("2021")


def main():
    categorize()
    # remove_empty_dirs()


def remove_empty_dirs(path=PATH):
    for day in path.iterdir():
        if not day.is_dir():
            continue

        try:
            next(day.iterdir())
        except StopIteration:
            day.rmdir()
            print(f"Removing {day}")


def categorize(path=PATH):
    files = {x for x in path.iterdir() if x.is_file()}
    buckets = {}

    for i in range(25, 0, -1):
        bucket = {x for x in files if f"{i}" in x.name}
        files = files - bucket
        buckets[i] = bucket

    pprint(buckets)
    # move all the files to the buckets
    for day, files in buckets.items():
        day_path = path / f"{day}"
        if files:
            day_path.mkdir(exist_ok=True)

        for file in files:
            file.rename(day_path / file.name)


if __name__ == "__main__":
    main()
