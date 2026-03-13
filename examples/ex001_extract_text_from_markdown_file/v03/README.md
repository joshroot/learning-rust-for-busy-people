# Extract text from Markdown file, v03

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

The program panics (i.e. exits with an error). A single command-line argument is required when running this program. This means that the `args` vector that contains the command-line arguments should have a length of 2. (The first element is the name of the program binary. The second element is the path to the Markdown file.) If the program is run without any command-line arguments, the length of the `args` vector is 1. In the code, the length of the `args` vector is checked and if it is less than 2, the `panic!` macro is called with the message "Not enough command-line arguments":

Command:
```
cargo run
```

Snippet of output:
```
thread 'main' (27631) panicked at src/lib.rs:8:9:
Not enough command-line arguments
```

> **_NOTE:_** _In this version of the example code, the program panics in this scenario because the code explicitly calls the `panic!` macro if the length of the `args` vector is less than 2. This is in contrast to the previous version of the example code when the `panic!` macro was not called explicitly in the code and the program panicked as a result of trying to get the value of an array element via an out-of-bounds index._


## Scenario 4

The program terminates early and prints an error message. The command-line argument must be a path to a file that exists. If the program is run with a command-line argument, but the file path that is provided does not exist, the `read_to_string` function returns an error. Use of the `?` operator after the `read_to_string` function causes the error to be returned early from the enclosing function, `run`, in the library crate. The error is then propagated to the code in the binary crate that called the `run` function. The user sees a custom message (defined in the binary crate) that includes the error message:

Command:
```
cargo run -- does_not_exist.md
```

Snippet of output:
```
Application error: No such file or directory (os error 2)
```

> **_NOTE:_** _In this version of the example code, the program no longer panics in this scenario because the error is now propagated and handled instead of using the `expect` method._

## Scenario 5

The program terminates early and prints an error message. The file must be non-empty. If the program is run with a command-line argument and the file path that is provided does exist but the file is empty, the `next` method on the iterator over the lines of the string returns the `None` variant of the `Option` enum. The `ok_or` method is used to transform the `Option` enum into a `Result` enum, where a `None` variant is mapped to an `Err` variant that contains the message "File is empty". Use of the `?` operator after the `ok_or` method causes the error to be returned early from the enclosing function, `run`, in the library crate. The error is then propagated to the code in the binary crate that called the `run` function. The user sees a custom message (defined in the binary crate) that includes the error message:


Command:
```
cargo run -- ../data/empty_file.md
```

Snippet of output:
```
Application error: File is empty
```

> **_NOTE:_** _In this version of the example code, the program no longer panics in this scenario because the error is now propagated and handled instead of using the `expect` method._
