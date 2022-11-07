<!-- ![workflow](https://img.shields.io/github/workflow/status/xtekc/shdel/audit/main?label=audits) ![workflow](https://img.shields.io/github/workflow/status/xtekc/shdel/cli-test?label=unit-tests) ![license](https://img.shields.io/github/license/xtekc/shdel) -->

# **shdel**
Delete data from shell history file(s). <br> 

| **Supported Shells** |
|----------------------|
|    [&check;] Bash    |
|    [&check;] Zsh     |
|    [~] Fish    |

## **Features**
- checks if the corresponding file exists
- if the file exists, it deletes the data
- if the file does not exist, it returns an error message

## **Install**
x86_64-linux-gnu
```
curl -L https://raw.githubusercontent.com/xTeKc/shdel/main/scripts/local/install-x86_64-linux-gnu.sh | bash
```
x86_64-apple-darwin
```
curl -L https://raw.githubusercontent.com/xTeKc/shdel/main/scripts/local/install-x86_64-apple-darwin.sh | bash
``` 

<!-- **Run with Bash:**
---
```
sh <(curl https://shdel.onrender.com/sh)
``` -->

<!-- **Run with Powershell:**
---
```
iwr -useb https://shdel.onrender.com/ps | iex
``` -->

## **Usage**
- `shdel -b` &nbsp; deletes bash history data from **.bash_history** file
- `shdel -z` &nbsp; deletes zsh history data from **.zsh_history** or **.zhistory** file
- `shdel -f` &nbsp; deletes fish history data from **.fish_history** file

```
shdel 0.1.0
Delete data from shell history file(s).

USAGE:
    shdel [OPTION]

OPTIONS:
    -b, --bash                      Delete bash data
    -z, --zsh                       Delete zsh data
    -f, --fish                      Delete fish data
    -h, --help                      Print help information
    -v, --version                   Print version information
```