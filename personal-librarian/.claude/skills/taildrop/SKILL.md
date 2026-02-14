---
name: taildrop
description: Check for files received via Tailscale Taildrop and help add them to the Obsidian vault
argument-hint: "[optional: specific filename to process]"
allowed-tools: Bash(sudo /usr/bin/tailscale file get *), Bash(ls *), Bash(file *), Bash(mkdir *), Read, Grep, Glob, Write, Edit
---

# Taildrop File Receiver

Receive files sent via Tailscale Taildrop and help the user add them to the vault.

## Steps

1. **Receive pending files**: Run `sudo /usr/bin/tailscale file get /tmp/taildrop-inbox/` to pull any pending transfers. Create the directory first if it doesn't exist.

2. **List received files**: Show the user what was received — filenames, sizes, and file types (use `file` command to identify them).

3. **Process files**: For each file (or the one specified in `$ARGUMENTS`):
   - If the file is a text-based format (markdown, text, PDF, etc.), read its contents and summarize it for the user.
   - If the file is binary (image, archive, etc.), describe what it is based on filename and file type detection.

4. **Propose vault placement**: Following the CLAUDE.md knowledge base rules:
   - Search the vault for related existing content.
   - Suggest which folder the file belongs in (or if it should be added to an existing note).
   - Propose relevant tags and links.
   - **Always ask the user before making any changes.**

5. **Store in vault**: After user approval, move or copy the file into the vault and create/update any notes, tags, or links as agreed.

6. **Clean up**: After all files are processed, let the user know if any files remain in `/tmp/taildrop-inbox/`.
