# Helpers

This repo contains helper scripts that I make for myself, in case they might be
helpful to others.

## Current inventory

- `conserve`: A program for managing battery conservation mode on supported
  Lenovo laptops.
  **NEW**: `conserve` is now written in Rust rather than C! This gives us a
  richer command-parsing system (with partial matching!), as well as
  memory-safety guarantees that are very helpful to have for an SUID binary.
- `notify`: A (very) simple wrapper script that executes a command, then rings
  your terminal bell when it finishes. Potentially helpful for long-running
  commands, like installers.
- `stmux`: A semi-intelligent script to manage shared `tmux` sessions for
  multiple users. Creates sockets in a known, accessible location, and searches
  for existing sessions by name before creating a new one.
- `nvim-hosted`: A script to start Neovim in a "host" terminal. This is
  required for Neovim to display itself, and can simplify things like window
  manager commands.
