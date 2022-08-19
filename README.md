# Helpers

This repo contains helper scripts that I make for myself, in case they might be
helpful to others.

## Current inventory

- `conserve`: A script for managing battery conservation mode on supported
  Lenovo laptops.
- `notify`: A (very) simple wrapper script that executes a command, then rings
  your terminal bell when it finishes. Potentially helpful for long-running
  commands, like installers.
- `stmux`: A semi-intelligent script to manage shared `tmux` sessions for
  multiple users. Creates sockets in a known, accessible location, and searches
  for existing sessions by name before creating a new one.
