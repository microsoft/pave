# pave

`pave` is a cross-platform CLI for managing your PATH. It supports Windows, macOS, and Linux across multiple shells.

## Getting Started

### Installation

#### homebrew (macOS/linux)

```sh
brew tap microsoft/pave https://github.com/microsoft/pave
brew install pave
```

#### build from source

```sh
git clone --depth 1 https://github.com/microsoft/pave.git ~/.pave && cargo install --path ~/.pave && rm -rf ~/.pave
```

### Shell Plugin

On **Windows**, pave writes directly to the registry and broadcasts `WM_SETTINGCHANGE`.

On **macOS/Linux**, add the plugin to your shell config:

<details>
<summary>Bash</summary>

Add to `~/.bashrc`:

```sh
export PATH="$(pave env bash)"
```

</details>

<details>
<summary>Zsh</summary>

Add to `~/.zshrc`:

```sh
export PATH="$(pave env zsh)"
```

</details>

<details>
<summary>Fish</summary>

Add to `~/.config/fish/config.fish`:

```fish
set -gx PATH (pave env fish)
```

</details>

<details>
<summary>PowerShell</summary>

Add to your profile (`$PROFILE`):

```powershell
$env:PATH = (pave env pwsh)
```

</details>

<details>
<summary>Xonsh</summary>

Add to `~/.xonshrc`:

```python
$PATH = $(pave env xonsh).strip().split('\n')
```

</details>

<details>
<summary>Nushell</summary>

Add to your env config (`$nu.env-path`):

```nu
$env.PATH = (pave env nushell | lines)
```

</details>

### Usage

| Action | Command |
| --- | --- |
| Add current directory | `pave add .` |
| Add specific directory | `pave add /usr/local/go/bin` |
| Search & add executable | `pave add rustup` |
| Remove directory | `pave remove /usr/local/go/bin` |
| Remove interactively | `pave remove` |
| List PATH entries | `pave list` |
| Search PATH | `pave search node` |

## Integrations

- [bash](https://www.gnu.org/software/bash/)
- [zsh](https://www.zsh.org/)
- [fish](https://github.com/fish-shell/fish-shell)
- [pwsh](https://github.com/PowerShell/PowerShell)
- [xonsh](https://xon.sh/)
- [nushell](https://www.nushell.sh/)

## Contributing

This project welcomes contributions and suggestions. Most contributions require you to agree to a
Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us
the rights to use your contribution. For details, visit https://cla.opensource.microsoft.com.

When you submit a pull request, a CLA bot will automatically determine whether you need to provide
a CLA and decorate the PR appropriately (e.g., status check, comment). Simply follow the instructions
provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/).
For more information see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or
contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.

## Trademarks

This project may contain trademarks or logos for projects, products, or services. Authorized use of Microsoft
trademarks or logos is subject to and must follow
[Microsoft's Trademark & Brand Guidelines](https://www.microsoft.com/en-us/legal/intellectualproperty/trademarks/usage/general).
Use of Microsoft trademarks or logos in modified versions of this project must not cause confusion or imply Microsoft sponsorship.
Any use of third-party trademarks or logos are subject to those third-party's policies.
