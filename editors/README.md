# Editor integrations

| Editor | Distribution | Source |
|---|---|---|
| **VS Code** | [Marketplace](https://marketplace.visualstudio.com/items?itemName=refract.refract) (once published) | [`vscode/`](vscode/) — full extension with status bar, output channel, runDoctor command |
| **Neovim** | [`hrtsx/refract.nvim`](https://github.com/hrtsx/refract.nvim) — separate plugin repo | [`neovim/`](neovim/) — minimal `lspconfig` snippet for users who don't use a plugin manager |
| **Zed** | Zed Extension Registry (once published) | [`zed/`](zed/) — Rust extension scaffold |
| **Helix** | Built-in via `languages.toml` | (see main [README](../README.md#helix)) |
| **Emacs** | Built-in via `eglot` | [`emacs/`](emacs/) |
| **Sublime Text** | Built-in via [LSP package](https://lsp.sublimetext.io/) | (see main [README](../README.md#sublime-text)) |

## Build the VS Code extension locally

```sh
cd vscode
npm install
npm run build
# load in VS Code: Extensions panel → "..." menu → Install from VSIX
# or: open the dir in VS Code and press F5 to launch a dev host
```

## Build the Zed extension locally

```sh
cd zed
# In Zed: cmd-shift-p → "extensions: install dev extension" → select editors/zed/
```

## Notes

- **VS Code** sends `disableTypeChecker`, `typeCheckerSeverity`, and other settings via `initializationOptions`. They hot-reload on `workspace/didChangeConfiguration`.
- **Neovim** plugin is in a separate repo for compatibility with plugin managers (lazy.nvim, packer, etc.). The local `neovim/init.lua` is a manual fallback.
- **Zed** extension currently registers Refract as a Ruby LSP. Per-project `.zed/settings.json` overrides are honored.
