## MCP Setup for Athenos AI (Windows + macOS first, Linux later)

This project uses Model Context Protocol (MCP) servers to enable safe autonomy, fast local workflows, and high‑quality UI.

### 1) shadcn MCP (UI components)

- Purpose: browse/search/install component primitives via natural language to build a premium, consistent UI.
- Cursor config (already added):

```
.cursor/mcp.json
{
  "mcpServers": {
    "shadcn": {
      "command": "npx",
      "args": ["shadcn@latest", "mcp"]
    }
  }
}
```

- Enable in Cursor: Settings → MCP → toggle `shadcn` (should show a green dot once active).
- Optional project init (if needed by your client): `npx shadcn@latest mcp init --client cursor`
- Registries: configure `components.json` if you want additional registries (e.g. `@acme`).

### 2) GitHub MCP (repo ops, PRs, code context) — via Docker MCP Toolkit Gateway

- Purpose: PRs, issues, code search/context, CI triggers from within the AI assistant.
- Implementation: use Docker’s MCP Gateway to manage GitHub Official server. Docs: https://docs.docker.com/ai/mcp-catalog-and-toolkit/toolkit/

- Configuration (added to `.cursor/mcp.json`):

```
{ "mcpServers": {
  "docker-mcp-gateway": {
    "command": "docker",
    "args": ["mcp", "gateway", "run"],
    "type": "stdio"
  },
  "github": {
    "command": "docker",
    "args": ["mcp", "server", "enable", "github-official"]
  }
} }
```

- Auth: use Docker MCP Toolkit OAuth for GitHub (recommended). In Docker Desktop → MCP Toolkit → Catalog → add “GitHub Official” → Configure → OAuth.
- Alternative: export `GITHUB_PERSONAL_ACCESS_TOKEN` in OS env and restart Cursor.

- Optional hardening:
  - In Docker Desktop, set read-only/lockdown options on the GitHub Official server if available.

### 3) DesktopCommanderMCP (local ops, fast loops) — CHOSEN: `desktop-commander-mcp`

- Purpose: local terminal/file ops/refactors without cloud latency; ideal for Windows/mac.
- Configuration (added to `.cursor/mcp.json`):

```
{ "mcpServers": { "desktop-commander": { "command": "npx", "args": ["desktop-commander-mcp"] } } }
```

### 4) Docker MCP (sandboxed runs) — CHOSEN: `mcp-docker`

- Purpose: build/run containers for sandboxed automation tests (“try before apply”).
- Prereqs: Docker Desktop (Windows/macOS). Enable Kubernetes only if needed.
- Configuration (added to `.cursor/mcp.json`):

```
{ "mcpServers": { "docker": { "command": "npx", "args": ["mcp-docker"] } } }
```

### 5) MCP Compass (discover more MCPs quickly) — CHOSEN: `mcp-compass`

- Purpose: discover and plug new servers faster as the stack evolves.
- Configuration (added to `.cursor/mcp.json`):

```
{ "mcpServers": { "mcp-compass": { "command": "npx", "args": ["mcp-compass"] } } }
```

> Note: Brave Search MCP was intentionally omitted per request.

---

## Platform prerequisites

- Windows (PowerShell): winget install Git, Node LTS, Python 3.11, Rust, Docker Desktop. Optional: PostgreSQL, Redis, Ollama, Playwright dependencies.
- macOS (Homebrew): git, node@lts, python@3.11, rust, docker, postgresql, redis, ollama, playwright deps.

## Recommended enablement order

1) Docker MCP Gateway, GitHub Official, shadcn MCP, DesktopCommanderMCP  
2) Docker MCP (for sandboxed runs if needed directly)  
3) MCP Compass

---

## Next actions (to finalize non‑shadcn servers)

1) Set your GitHub token in your secret manager (scopes: `repo`, `workflow` if needed).
2) Ensure Docker Desktop is installed and running (Windows/macOS).
3) Enable all servers in Cursor Settings → MCP and verify green status.
4) I’ll validate tool availability and document any required env vars in a `.env.example`.


