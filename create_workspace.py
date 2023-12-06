import argparse
import shutil
from logging import INFO, basicConfig, getLogger
from pathlib import Path

BOILERPLATE_PATH = Path("boilerplate.py")
SOLUTION_NAME = "solution.py"
log = getLogger(__name__)


def main(args: argparse.Namespace) -> None:
    from aocd import exceptions, models, utils
    from dotenv import load_dotenv

    load_dotenv()

    if not args.quiet:
        basicConfig(level=INFO)

    year, day = get_date(args.year, args.day, args.tomorrow)

    # create puzzle folder
    path = Path(f"{year}/{day}/")
    path.mkdir(parents=True, exist_ok=True)

    # copy boilerplate and rename to solution.py
    solution_path = path / SOLUTION_NAME

    if solution_path.is_file():
        log.warning(f"{solution_path} already exists, skipping")
    else:
        log.info(f"copying {BOILERPLATE_PATH} to {solution_path}")
        shutil.copyfile(BOILERPLATE_PATH, solution_path)

    if not args.no_input:
        if args.session is None:
            user = models.default_user()
        else:
            user = models.User(token=args.session)

        puzzle = models.Puzzle(year=year, day=day, user=user)

        try:
            input_data = puzzle.input_data
        except exceptions.PuzzleLockedError:
            if args.no_block:
                raise

            utils.blocker(quiet=args.quiet, until=(year, day))
            input_data = puzzle.input_data

        log.info(f"loading puzzle input for {year=}, {day=}")

        # write input data to input.txt
        input_path = path / "input.txt"
        input_path.write_text(input_data)

    log.info("done")


def get_date(year=None, day=None, tomorrow=False):
    from aocd import get

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

    return year, day


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        prog="create_workspace",
        description=(
            "Create workspace for Advent of Code puzzle. "
            "Running with --tomorrow will block until puzzle is available. "
            "This Behavior can be disabled with the --no-block flag."
        ),
    )

    parser.add_argument(
        "-y",
        "--year",
        type=int,
        default=None,
        help="year of the puzzle, defaults to most recent year",
    )
    parser.add_argument(
        "-d",
        "--day",
        type=int,
        default=None,
        help="day of the puzzle, defaults to current day",
    )
    parser.add_argument(
        "-n",
        "--no-block",
        action="store_true",
        help="do not block until puzzle is unlocked",
    )
    parser.add_argument(
        "-i",
        "--no-input",
        action="store_true",
        help="do not download puzzle input",
    )
    parser.add_argument(
        "-q",
        "--quiet",
        action="store_true",
        help="do not print status messages",
    )
    parser.add_argument(
        "-t",
        "--tomorrow",
        action="store_true",
        help="get tomorrow's puzzle instead of today's",
    )
    parser.add_argument(
        "-s",
        "--session",
        type=str,
        default=None,
        help="session cookie for aocd",
    )
    args = parser.parse_args()
    main(args)
