# Refract for Zed

Zed extension that registers [Refract](https://github.com/hrtsx/refract) as a Ruby language server.

## Requirements

- Zed 0.151.0+
- The `refract` binary on `PATH` (`brew install hrtsx/refract/refract` or download from [Releases](https://github.com/hrtsx/refract/releases/latest))

## Installation

### From the Zed extension registry

Once published:

```
zed: extensions
search: refract
install
```

### As a dev extension (for testing)

```sh
git clone https://github.com/hrtsx/refract
zed: install dev extension
# point Zed at editors/zed/
```

## What it does

Registers the `refract` binary as a Ruby language server. Zed will spawn `refract` over stdio for any opened `.rb` / `.erb` / `.haml` file in a project that contains a `Gemfile` or `.git`.

## Configuration

Per-project overrides in `.zed/settings.json`:

```json
{
  "lsp": {
    "refract": {
      "initialization_options": {
        "disableGemIndex": false,
        "disableRubocop": false,
        "logLevel": 3,
        "maxFileSizeMb": 8
      }
    }
  }
}
```

See the main repo's [Configuration table](https://github.com/hrtsx/refract#configuration) for all options. They hot-reload on `workspace/didChangeConfiguration`.

## Without this extension

If you'd rather configure Zed manually (no extension required), add to `.zed/settings.json`:

```json
{
  "lsp": {
    "refract": { "binary": { "path": "refract" } }
  },
  "languages": {
    "Ruby": { "language_servers": ["refract"] }
  }
}
```

## License

MIT — same as Refract.
