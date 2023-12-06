# My Personal AOC setup

## USAGE

1. Clone the repo.
2. Replace `TOKEN` in `./.env` file with advent of code session
   id ([how to get](https://github.com/wimglenn/advent-of-code-wim/issues/1)).

This token is used to automatically get personal inputs from advent of code server. You can disable this in by passing
--no-input to create-workspace. DO NOT COMMIT THIS TOKEN TO A PUBLIC REPOSITORY.

## Files

- `boilerplate.py`: this is the solution boilerplate that will be copied as solution.py file into the solution folder.

- `create_workspace.py`:
    1. create puzzle folder (`./{year}/{day}` by default)
    2. copy boilerplate file into the folder
    3. wait until puzzle is opened to download the input file

- `organize.py`: helper for organizing messy solution files into `{year}/{day}` format.

Run boilerplate.py with --tomorrow a few minutes before the puzzle. Then open the website, wait until its unlocked. When
you're done reading the puzzle, solution boilerplate and input file are ready in the folder for you.
