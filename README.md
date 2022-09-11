![workflow](https://github.com/xTeKc/ShellDel/actions/workflows/ci.yml/badge.svg)

# **ShellDel**
Delete data from shell history file(s). <br> 

| **Supported Shells** |
|----------------------|
|    [&check;] Bash    |
|    [&check;] Zsh     |
|    [&check;] Fish    |

## **Features**
- checks if the corresponding file exists
- if the file exists, it deletes the data
- if the file does not exist, it returns an error message

## **Get Started**
- `clone repo`
- `cargo build`

## **How to use**
- `cargo run -q -- -b` $~$ (deletes bash history data from **.bash_history** file)
- `cargo run -q -- -z` $~$ (deletes zsh history data from **zsh_history** or **.zhistory** file)
- `cargo run -q -- -f` $~$ (deletes fish history data from **.fish_history** file)

```
shd 0.1.0
Delete data from shell history file(s).
Author: xTeKc

USAGE:
    shd [OPTION]

OPTIONS:
    -b, --bash                      Delete bash data
    -z, --zsh                       Delete zsh data
    -f, --fish                      Delete fish data
    -h, --help                      Print help information
    -v, --version                   Print version information
```