from logging import getLogger, basicConfig, INFO
from pathlib import Path
import shutil
import argparse
from aocd import models, get, exceptions, utils
from dotenv import load_dotenv


basicConfig(level=INFO)
BOILERPLATE_PATH = Path("boilerplate.py")
log = getLogger(__name__)
load_dotenv()


def main(args):
    session = None
    year = args.year
    day = args.day
    block = not args.no_block
    quiet = args.quiet
    tomorrow = args.tomorrow

    if session is None:
        user = models.default_user()
    else:
        user = models.User(token=session)

    if day is None:
        day = get.current_day()
        if tomorrow:
            day += 1

        log.info("chosen day=%s", day)

    if not 0 <= day <= 25:
        raise ValueError(f"day must be between 0 and 25, got {day}")

    if year is None:
        year = get.most_recent_year()
        log.info("most recent year=%s", year)

    puzzle = models.Puzzle(year=year, day=day, user=user)

    path = Path(f"{year}/{day}/")
    path.mkdir(parents=True, exist_ok=True)

    # copy boilerplate and rename to solution.py
    solution_path = path / "solution.py"
    log.info(f"copying {BOILERPLATE_PATH} to {solution_path}")
    if solution_path.is_file():
        log.warning(f"{solution_path} already exists, skipping")
    else:
        shutil.copyfile(BOILERPLATE_PATH, solution_path)

    try:
        input_data = puzzle.input_data
    except exceptions.PuzzleLockedError:
        if not block:
            raise

        utils.blocker(quiet=quiet, until=(year, day))
        input_data = puzzle.input_data

    log.info(f"loading puzzle input for {year=}, {day=}")

    # write input data to input.txt
    input_path = path / "input.txt"
    input_path.write_text(input_data)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        prog='create_workspace',
        description=(
            "Create workspace for Advent of Code puzzle. "
            "Running with no puzzle date will block until today's puzzle is available. "
            "This Behavior can be disabled with the --no-block flag."
        )
    )

    parser.add_argument("-y", "--year", type=int, default=None)
    parser.add_argument("-d", "--day", type=int, default=None)
    parser.add_argument("-n", "--no-block", action="store_true")
    parser.add_argument("-q", "--quiet", action="store_true")
    parser.add_argument("-t", "--tomorrow", action="store_true")
    args = parser.parse_args()
    main(args)
