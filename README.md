# Jac Extension for Zed

This extension provides support for the [Jac programming language](https://www.jac-lang.org/) in [Zed](https://zed.dev/), including Tree-sitter syntax highlighting, bracket matching, code outline, text objects, Python code injection, and Language Server Protocol (LSP) integration.

## Features

- **Syntax Highlighting**: Complete syntax highlighting powered by Tree-sitter (`tree-sitter-jac`).
- **Language Server Protocol (LSP)**: Automatic discovery and launch of `jac lsp` for code diagnostics, auto-completions, and navigation.
- **Python Code Injection**: Syntax highlighting for inline Python blocks (`::py:: ... ::py::`).
- **Bracket Matching & Auto-Closing**: Matching brackets and auto-closing quotes and comments (`#* ... *#`).
- **Outline & Structure**: Symbol outline for archetypes (`obj`, `node`, `edge`, `walker`, `class`), `enum`, and abilities (`can`, `def`).
- **Text Objects**: Outer and inner text objects for functions/abilities and classes/archetypes.

## Prerequisites

- Install `jac`:
  ```bash
  pip install jaclang
  ```
  Or visit [Jac Installation Guide](https://www.jac-lang.org/learn/installation/).

## Installation (Development)

To install this extension locally in Zed:

1. Open Zed.
2. Open the Command Palette (`Ctrl-Shift-P` or `Cmd-Shift-P`).
3. Run `zed: install dev extension`.
4. Select the directory containing this extension (`/home/eli/classes/cse449/zed-jac`).

## Configuration

By default, the extension discovers `jac` from your project virtual environment (`.venv`), your `$PATH`, or standard install paths (`~/.local/bin/jac`).

If you wish to configure a custom Jac binary or arguments, add the following to your Zed `settings.json`:

```json
{
  "lsp": {
    "jac": {
      "binary": {
        "path": "/custom/path/to/jac",
        "arguments": ["lsp"]
      }
    }
  }
}
```
