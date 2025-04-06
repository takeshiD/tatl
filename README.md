<div align="center">
  <div>
    <h1 align="center">TATL</h1>
  </div>
    <p>`tatl`(pronounced same as `chat`) is a fairy companion watches over your commits, so automatically generates commit messages.</p>
</div>

# Features

- Generate commit messages
- Multi supported LLM Providers
- Integrated `lazygit`, `nvim`(Of course, `tatl` can be used as CLI Tool)
# Installation

## Package Managers

### Debian

> [!WARNING]
> Not yet prepared
```sh
$ sudo apt install tatl
```

### Arch Linux
> [!WARNING]
> Not yet prepared
```sh
$ sudo pacman -S tatl
```

## Prebuild Binary
> [!WARNING]
> Not yet prepared

## Build from Source

### Requirements
- `rust`, `cargo` >= 1.70

### Build
```sh
$ cargo install --locked tatl
```

# Setup LLM

## Default
`tatl` detects environment variable for LLM Providers.

| Providers   | EnvVar              | Model              |
| ----------- | -----------------   | -------------      |
| OpenAI      | `OPENAI_API_KEY`    | `gpt-4o-mini`      |
| Anthropic   | `ANTHROPIC_API_KEY` | `claude-3-5-haiku` |
| Google      | `GOOGLE_API_KEY`    | `gemini-2.0-flash` |


# Usage as CLI Tool

## Setting LLM Provider Model
```sh
$ tatl config set model=gpt-4o-mini
✓  Setting Provider as gpt-4o-mini
```

## Interactive Mode
Interactively suggest and select a commit message

```sh
$ git add <files>
$ tatl
Model: gpt-4o-mini
Suggesting Messages...
<Space>: select  <Enter>: confirm <Esc>: cancel
  ✓   feat: implemented linked-list class, modified interface
      feat: implemented linked-list class, modified interface
> ✓   docs: modified docstring about linked-list, and related part in README.md
      Cancel

# Press `Enter`
Add Prompt? > refer to implemented interface for singly and doubly linked-list
Regenerate message based on selected messages....
Suggestion Message
<Enter>: confirm  <e>: edit

feat: implemented linked-list class, modified interface, and modified docstring.

* implemented linked-list class, modified interface
* modified docstring about linked-list, and related part in README.md
* imeplemented interface for singly and doubly linked-list

# Press `Enter`
Commited
```

## Non-Interactive Mode
In interactive mode you can choose from multiple messages, but in non-interactive mode the message that is rated as best explaining the differences will be selected.

```sh
$ git add <files>
$ tatl --no-interactive
✓ Commited Message
feat: implemented linked-list class, modified interface
done
```

# Usage as integrated `lazygit`
`tatl` provides output-format for `lazygit` custom menu.

> [!WARNING]
> Not yet prepared

```sh
$ tatl --output-format=lazygit
```

