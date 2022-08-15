![workflow](https://github.com/xTeKc/ShellDel/actions/workflows/ci.yml/badge.svg)

# ShellDel
Delete data from shell history file(s). <br>
It currently only deletes data from **bash** and **zsh** terminal history files

__Usage__ : 
- `clone repo`
- `cargo build`
- `cargo run -q -- -b` $~$ to delete bash history data from **.bash_history** file
- `cargo run -q -- -z` $~$ to delete zsh history data from **.zhistory** file
- **reset terminal** $~$ *(work in progress, to automatically reset)*

__Capabilities__ : 
- checks if the corresponding file exists
- if the file exists, it deletes the data
- if the file does not exist, it returns an error message

__Work in Progress__ :
- continue to improve user experience
- write a few more tests
- build for release