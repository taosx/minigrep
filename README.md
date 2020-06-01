# Experimenting: pretty errors messages for command line applications

Inspired by the error messages given I wanted to see if it's possible to have the same for command line applications.

## Example (end goal)

```
> minigrep vindia

usage of minigrep: minigrep vindia <path to file>
                                   ^^^^^^^^^^^^^^ - [error] The file location to be grep-ed is missing
                            ^^^^^^ - [info] pattern used to search in file

Hints:
You need to pass a file in order to find pattern and print them.
```

=============================

```sh
> minigrep vindia /logs/requests.log

(show results of of pattern matches vindia on log file)
```

=============================

```sh
> minigrep vindia /logs/requests.log badargument_notafile

usage of minigrep: minigrep vindia /logs/requests.log nothing_here
                                                      ^^^^^^^^^^^^ [error] File or folder doesn't exist at location.
                                   ^^^^^^^^^^^^^^^^^^ [info] File argument [ok]
                            ^^^^^^ [info] pattern to match in file/s

Hints:
Last argument is not a file or directory, please check if the location is right:
location: /home/assos/right_path/nothing_here
```

============================

## Methods

Currently this is hardcoded but looking for new methods.

### Read execution from shell history

1. Detect shell used
2. Read history file (\$HISTFILE for bash and zsh, try to find for fish)
3. Identify arguments and anotate

#### Questions

- What happens when there are multiple users on the same PC?
- What in case history file not found due to configuration or security?

### Hardcoded

1.
