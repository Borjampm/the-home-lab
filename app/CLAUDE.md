# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

The Home Lab is a personal home lab project for connecting computers and utilities. This `app/` directory is the application component of the larger `the-home-lab` monorepo.

## Repository Structure

The parent repository (`the-home-lab/`) contains:
- `app/` — Application code (this directory)
- `research/` — Research agents and reports (has its own CLAUDE.md with research methodology guidelines)

## Current State

This application directory is in early development. No build system, package manager, or test framework is configured yet. Documentation lives in `docs/`.

## Documentation Convention

Documentation uses an index-based architecture:
- Each folder contains a `README.md` that serves as an index for that directory
- The README links to files in the same folder and/or routes to subdirectories
- Each README includes a brief description of what the folder contains
- Subdirectories follow the same pattern with their own `README.md`

When starting work on a task, check `docs/` for existing documentation that may provide relevant context.

## Maintaining This File

As we work together, proactively propose updates to this CLAUDE.md when you notice:
- Project conventions or patterns emerging from the code
- User preferences for tooling, style, or workflow
- Architecture decisions worth preserving for future sessions
- New build/test/lint commands as they are introduced
- Any recurring context that would help future instances ramp up faster
