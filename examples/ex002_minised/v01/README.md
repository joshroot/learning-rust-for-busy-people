# minised, v01

## Description

* Mini version of the `sed` command-line utility.
* Simple implementation of the substitute (`s` command) functionality.
* Allows for substitution of patterns that consist of a fixed string (i.e. regular expressions are not supported).
* Reads contents from a file path (passed as a command-line argument). If a file path is not provided then reads contents from standard input (STDIN).
* Prints the modified contents to standard output (STDOUT) by default.
* Modifies the file contents "in-place" if the `-i` argument is passed as the first command-line argument.

## Build

Build the debug executable:

Command:
```
cargo build
```

## Scenario 1

The program runs successfully and prints the transformed text to standard output. Global replacement of "Monday" with "Tuesday". STDOUT and STDERR are both printed to the command-line shell:

Command:
```
./target/debug/ex002_minised_v01 's/Monday/Tuesday/g' ../data/mondays.csv
```

## Scenario 2

The program runs successfully and prints the transformed text to standard output. In each line, replace the 2nd instance of "Monday" with "Tuesday". STDOUT and STDERR are both printed to the command-line shell:

Command:
```
./target/debug/ex002_minised_v01 's/Monday/Tuesday/2' ../data/mondays.csv
```

## Scenario 3

The program runs successfully. Inclusion of the `-i` option switches behavior to modify the file in-place, so nothing is printed to standard output. STDERR is printed to the command-line shell.

Command:
```
./target/debug/ex002_minised_v01 -i 's/Monday/Tuesday/g' ../data/mondays.csv
```

## Scenario 4

The program terminates early and prints an error message.

Command:
```
./target/debug/ex002_minised_v01
```

Snippet of output:
```
Argument parsing error: Not enough command-line arguments
```

## Scenario 5

The program terminates early and prints an error message.

Command:
```
./target/debug/ex002_minised_v01 -v 's/Monday/Tuesday/g' ../data/mondays.csv
```

Snippet of output:
```
Argument parsing error: The first command-line argument is not valid, expected `-i`
```

## Scenario 6

The program terminates early and prints an error message.

Command:
```
./target/debug/ex002_minised_v01 's/Monday/Tuesday/g' ../data/mondays.csv --unneeded-argument
```

Snippet of output:
```
Argument parsing error: The first command-line argument is not valid, expected `-i`
```

## Scenario 7

The program terminates early and prints an error message.

Command:
```
./target/debug/ex002_minised_v01 -i 's/Monday/Tuesday/g' ../data/mondays.csv --unneeded-argument
```

Snippet of output:
```
Argument parsing error: Too many command-line arguments
```

## Scenario 8

The program terminates early and prints an error message.

Commands:
```
./target/debug/ex002_minised_v01 's/Monday/Tuesday/' ../data/mondays.csv

./target/debug/ex002_minised_v01 's/Monday/Tuesday/a' ../data/mondays.csv

./target/debug/ex002_minised_v01 's/Monday/Tuesday/0' ../data/mondays.csv
```

Snippet of output:
```
minised script parsing error: Flag must be 'g' or an integer greater than 0
```

## Scenario 9

The program terminates early and prints an error message.

Command:
```
./target/debug/ex002_minised_v01 's/Monday/Tuesday' ../data/mondays.csv
```

Snippet of output:
```
minised script parsing error: Invalid script format, expected 's/pattern/replacement/flag'
```

## Scenario 10

The program terminates early and prints an error message.

Command:
```
./target/debug/ex002_minised_v01 '/Monday/Tuesday/' ../data/mondays.csv
```

Snippet of output:
```
minised script parsing error: Invalid command name, expected "s"
```

## Tests

Run unit tests (21 total):

Command:
```
cargo test
```
