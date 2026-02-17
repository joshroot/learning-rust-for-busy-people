# Extract text from Markdown file, v02

## Description

* Read the contents of a Markdown file.
* The file path is specified via a command-line argument.
* Get the text of the first line (which we assume contains the title).
* Remove the leading `# ` characters from the text of the first line, leaving just the title text.
* Print the title text to standard output.

## Scenario 1

The program runs successfully. STDOUT and STDERR are both printed to the command-line shell:

Command:
```
cargo run -- ../data/musical_instruments.md
```

## Scenario 2

The program runs successfully. Redirect STDOUT and STDERR to separate text files (STDOUT and STDERR have file descriptors of 1 and 2, respectively):

Command:
```
cargo run -- ../data/musical_instruments.md 1> stdout.txt 2> stderr.txt
```

## Scenario 3

The program panics (i.e. exits with an error). A single command-line argument is required when running this program. If the program is run without any command-line arguments, trying to get the value of the element at index 1 in the `args` vector causes an error:

Command:
```
cargo run
```

Snippet of output:
```
thread 'main' (120045) panicked at src/main.rs:10:26:
index out of bounds: the len is 1 but the index is 1
```

## Scenario 4

The program panics. The command-line argument must be a path to a file that exists. If the program is run with a command-line argument, but the file path that is provided does not exist, the `read_to_string` function returns an error. The message passed to the `expect` method is printed. Although the user sees a customized message that explains why the program crashed (`Could not read the file`), the crash itself is not prevented:

Command:
```
cargo run -- does_not_exist.md
```

Snippet of output:
```
thread 'main' (120568) panicked at src/lib.rs:7:10:
Could not read the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

## Scenario 5

The program will panics. The file must be non-empty. If the program is run with a command-line argument and the file path that is provided does exist but the file is empty, the `next` method on the iterator over the lines of the string returns an error. The message passed to the `expect` method is printed. Although the user sees a customized message that explains why the program crashed (`File is empty`), the crash itself is not prevented:

Command:
```
cargo run -- ../data/empty_file.md
```

Snippet of output:
```
thread 'main' (121134) panicked at src/lib.rs:10:10:
File is empty
```
