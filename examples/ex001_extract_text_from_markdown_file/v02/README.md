# Extract text from Markdown file, v02

## Description

* Read the contents of a Markdown file specified via a command-line argument.
* Get the text of the first line (which we assume contains the title).
* Remove the leading `# ` characters from the text of the first line, leaving just the title text.
* Print the title text to standard output.

## Instructions

```
cargo run -- ../data/musical_instruments.md
```

## Considerations

* A single command-line argument is required when running this program.
* The command-line argument must be a path to a file that exists.
* The file must be non-empty.

## Possible reasons why the program panics

### Scenario 1

If the program is run without any command-line arguments, the program will panic (i.e. exit with an error):

Command:
```
cargo run
```

Snippet of output:
```
thread 'main' (136181) panicked at src/main.rs:11:26:
index out of bounds: the len is 1 but the index is 1
```

Since no command-line argument was provided, trying to get the value of the element at index 1 in the `args` vector causes an error. The `args` vector has a length of 1 and contains the name of the program at index 0.

### Scenario 2

If the program is run with a command-line argument, but the file path that is provided does not exist, the program will panic:

Command:
```
cargo run -- does_not_exist.md
```

Snippet of output:
```
thread 'main' (145807) panicked at src/main.rs:18:10:
Could not read the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

When the `read_to_string` function returns an error, the message passed to the `expect` method is printed. Although the user sees a customized message that explains why the program crashed, which is `Could not read the file`, the crash itself is not prevented.

### Scenario 3

If the program is run with a command-line argument and the file path that is provided does exist but the file is empty, the program will panic:

Command:
```
cargo run -- ../data/empty_file.md
```

Snippet of output:
```
thread 'main' (157688) panicked at src/main.rs:21:10:
File is empty
```

When the `next` method on the iterator over the lines of the string returns an error, the message passed to the `expect` method is printed. Although the user sees a customized message that explains why the program crashed, which is `File is empty`, the crash itself is not prevented.
