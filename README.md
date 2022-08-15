![workflow](https://github.com/xTeKc/ShellDel/actions/workflows/ci.yml/badge.svg)

# **ShellDel**
Delete data from shell history file(s). <br>
It currently only deletes data from **bash** and **zsh** terminal history files.

## **Usage** : 
- `clone repo`
- `cargo build`
- `cargo run -q -- -b` $~$ (deletes bash history data from **.bash_history** file)
- `cargo run -q -- -z` $~$ (deletes zsh history data from **.zhistory** file)
- **manually reset terminal** $~$ *(work in progress to reset terminal automatically)*

```
shelldel 0.1.0
Delete data from shell history file(s)

USAGE:
    shelldel [OPTIONS]

OPTIONS:
    -b, --bash       Delete bash data
    -h, --help       Print help information
    -V, --version    Print version information
    -z, --zsh        Delete zsh data
```

## **Capabilities** : 
- checks if the corresponding file exists
- if the file exists, it deletes the data
- if the file does not exist, it returns an error message

## **Work in Progress** :
- continue to improve user experience
- reset terminal automatically
- write a few more tests
- build for release