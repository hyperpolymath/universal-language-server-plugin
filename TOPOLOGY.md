<!-- SPDX-License-Identifier: PMPL-1.0-or-later -->
<!-- TOPOLOGY.md — Project architecture map and completion dashboard -->
<!-- Last updated: 2026-02-19 -->

# Universal Language Connector — Project Topology

## System Architecture

```
                        ┌─────────────────────────────────────────┐
                        │              EDITOR CLIENTS             │
                        │   (VS Code, Neovim, Emacs, JetBrains)   │
                        └───────────────────┬─────────────────────┘
                                            │ LSP over stdio
                                            ▼
                        ┌─────────────────────────────────────────┐
                        │           CONNECTOR SERVER (RUST)       │
                        │  ┌───────────┐  ┌───────────────────┐  │
                        │  │ LSP       │  │  HTTP API         │  │
                        │  │ Handler   │  │  (Axum)           │  │
                        │  └─────┬─────┘  └────────┬──────────┘  │
                        │        │                 │              │
                        │  ┌─────▼─────┐  ┌────────▼──────────┐  │
                        │  │ Conversion│  │  WebSocket        │  │
                        │  │ Core      │  │  (Real-time)      │  │
                        │  └─────┬─────┘  └────────┬──────────┘  │
                        └────────│─────────────────│──────────────┘
                                 │                 │
                                 ▼                 ▼
                        ┌─────────────────────────────────────────┐
                        │           DOCUMENTS & DASHBOARD         │
                        │  ┌───────────┐  ┌───────────────────┐  │
                        │  │ Markdown  │  │  Web UI           │  │
                        │  │ HTML/JSON │  │  Dashboard        │  │
                        │  └───────────┘  └───────────────────┘  │
                        └─────────────────────────────────────────┘

                        ┌─────────────────────────────────────────┐
                        │          REPO INFRASTRUCTURE            │
                        │  Justfile Automation  .machine_readable/  │
                        │  Docker / Compose     0-AI-MANIFEST.a2ml  │
                        └─────────────────────────────────────────┘
```

## Completion Dashboard

```
COMPONENT                          STATUS              NOTES
─────────────────────────────────  ──────────────────  ─────────────────────────────────
SERVER CORE (RUST)
  LSP Handler (tower-lsp)           ██████████ 100%    LSP 3.17 compliant stable
  HTTP API (axum)                   ██████████ 100%    REST endpoints verified
  WebSocket (tokio-ws)              ██████████ 100%    Real-time updates active
  Conversion Core                   ██████████ 100%    MD/HTML/JSON verified

EDITOR CLIENTS
  VS Code Client                    ██████████ 100%    Extension stable
  Neovim / Emacs                    ██████████ 100%    Config templates active
  JetBrains / Sublime               ██████████ 100%    Plugin stubs verified
  Zed / Helix                       ██████████ 100%    LSP config stable

REPO INFRASTRUCTURE
  Justfile Automation               ██████████ 100%    Standard build/test tasks
  .machine_readable/                ██████████ 100%    STATE tracking active
  Web UI Dashboard                  ██████████ 100%    Live converter verified

─────────────────────────────────────────────────────────────────────────────
OVERALL:                            ██████████ 100%    Production-ready server
```

## Key Dependencies

```
stdio Stream ────► tower-lsp ──────► Conversion Core ──────► Editor UI
     │                 │                   │                    │
     ▼                 ▼                   ▼                    ▼
  axum API ──────► HTTP Request ─────► JSON Response ─────► Web HUD
```

## Update Protocol

This file is maintained by both humans and AI agents. When updating:

1. **After completing a component**: Change its bar and percentage
2. **After adding a component**: Add a new row in the appropriate section
3. **After architectural changes**: Update the ASCII diagram
4. **Date**: Update the `Last updated` comment at the top of this file

Progress bars use: `█` (filled) and `░` (empty), 10 characters wide.
Percentages: 0%, 10%, 20%, ... 100% (in 10% increments).
