<div align="center">

# shdel
Delete data from shell history files.

<!-- <a href="https://crates.io/crates/shdel/"><img src="https://img.shields.io/crates/v/shdel?style=flat&amp;labelColor=032a1a&amp;color=065535&amp;logo=Rust&amp;logoColor=white" alt="Crate Release"></a> -->
<br>
<a href="https://github.com/xTekC/shdel/actions?query=workflow%3A%22Continuous+Integration%22"><img src="https://img.shields.io/github/actions/workflow/status/xTekC/shdel/ci.yml?branch=main&amp;style=flat&amp;labelColor=032a1a&amp;color=065535&amp;logo=GitHub%20Actions&amp;logoColor=white&amp;label=Build" alt="Continuous Integration"></a>
<a href="https://github.com/xTekC/shdel/actions?query=workflow%3A%22Continuous+Deployment%22"><img src="https://img.shields.io/github/actions/workflow/status/xTekC/shdel/cd.yml?style=flat&amp;labelColor=032a1a&amp;color=065535&amp;logo=GitHub%20Actions&amp;logoColor=white&amp;label=Release" alt="Continuous Deployment"></a>
<!-- <a href="https://docs.rs/shdel/"><img src="https://img.shields.io/docsrs/shdel?style=flat&amp;labelColor=032a1a&amp;color=065535&amp;logo=Rust&amp;logoColor=white" alt="Documentation"></a> -->

[![GitHub license](https://img.shields.io/github/license/xTekC/shdel.svg?style=flat&labelColor=032a1a&color=065535&logo=GitHub&logoColor=black&label=License)](https://github.com/xTekC/shdel/blob/main/LICENSE)

[![GitHub Sponsors](https://img.shields.io/badge/Sponsor-GitHub-purple?style=flat&labelColor=grey&color=8a63d2&logo=github&logoColor=white)](https://github.com/sponsors/xTekC)
[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A-Coffee-orange?style=flat&labelColor=grey&color=ff813f&logo=buy-me-a-coffee&logoColor=black)](https://www.buymeacoffee.com/xTekC)
[![Ko-fi](https://img.shields.io/badge/Support-Ko--fi-red?style=flat&labelColor=grey&color=f16061&logo=ko-fi&logoColor=white)](https://ko-fi.com/xTekC)

<a href="#features">Features</a> •
<a href="#installation">Installation</a> •
<a href="#usage">Usage</a> •
<a href="#contribution">Contribution</a> •
<a href="#donate">Donate</a>

</div>

## Features
|  |
|----------------------|
|    [&check;] Bash    |
|    [&check;] Zsh     |
|    [~] Fish    |

## Installation

**Cargo**

```
cargo install --git https://github.com/xTekC/shdel --branch main --locked --profile rel-opt
```

**Prebuilt Binary**
<br>

(For Android, use Termux: `https://f-droid.org/repo/com.termux_118.apk`)

Unix-Like [Install](https://github.com/xTeKc/shdel/blob/main/scripts/install.sh)<br>

```
curl -sSL https://raw.githubusercontent.com/xTeKc/shdel/main/scripts/install.sh | sh
```

Unix-Like [Remove](https://github.com/xTeKc/shdel/blob/main/scripts/remove.sh)

```
curl -sSL https://raw.githubusercontent.com/xTeKc/shdel/main/scripts/remove.sh | sh
```

## Usage

```
shdel -h
```

- `shdel -b` &nbsp; deletes bash history data from **.bash_history** file
- `shdel -z` &nbsp; deletes zsh history data from **.zsh_history** or **.zhistory** file
- `shdel -f` &nbsp; deletes fish history data from **.fish_history** file

## Contribution
Read the [Contributing Guide](CONTRIBUTING.md) before making a pull request.

## Donate
If you find shdel valuable and wish to support its progress, here are the various avenues through which you can extend your support:

- [GitHub Sponsors](https://github.com/sponsors/xTekC)
- [Buy Me A Coffee](https://www.buymeacoffee.com/xTekC)
- [Ko-fi](https://ko-fi.com/xTekC)

Your generous donations are invaluable and help fuel the ongoing development of shdel. <br>
Thank you!

<br>

Copyright (c) **xTekC** <br>
Licensed under [MPL-2.0](LICENSE)
