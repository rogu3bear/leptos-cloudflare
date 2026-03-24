#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

log() {
  printf '[init] %s\n' "$1"
}

# --- Input validation ---

if [ $# -ne 1 ]; then
  printf '[init] Usage: %s <project-name>\n' "$0" >&2
  exit 1
fi

PROJECT="$1"

if ! printf '%s' "$PROJECT" | grep -qE '^[a-z][a-z0-9-]*$'; then
  printf '[init] Error: project name "%s" is invalid.\n' "$PROJECT" >&2
  printf '[init] Must match ^[a-z][a-z0-9-]*$ (lowercase, start with letter, hyphens ok)\n' >&2
  exit 1
fi

WRANGLER_TOML="$ROOT_DIR/wrangler.toml"

if ! grep -q '00000000-0000-0000-0000-000000000000' "$WRANGLER_TOML"; then
  printf '[init] Error: wrangler.toml does not contain placeholder D1 IDs.\n' >&2
  printf '[init] This project may already be initialized. Refusing to overwrite.\n' >&2
  exit 1
fi

# --- Cross-platform sed -i ---

sedi() {
  if sed --version 2>/dev/null | grep -q GNU; then
    sed -i "$@"
  else
    sed -i '' "$@"
  fi
}

# --- Rewrite project identity ---

log "Rewriting Cargo.toml..."
sedi \
  -e "s/^name = \"leptos-cf\"/name = \"$PROJECT\"/" \
  -e "s/^description = \".*\"/description = \"$PROJECT\"/" \
  -e "s/^output-name = \"leptos-cf\"/output-name = \"$PROJECT\"/" \
  "$ROOT_DIR/Cargo.toml"

log "Rewriting wrangler.toml..."
sedi \
  -e "s/^name = \"leptos-cf\"/name = \"$PROJECT\"/" \
  -e "s/database_name = \"leptos-cf-db\"/database_name = \"$PROJECT-db\"/" \
  "$WRANGLER_TOML"

log "Rewriting src/app.rs identity..."
sedi \
  -e "s|text=\"Leptos CF Starter\"|text=\"$PROJECT\"|" \
  -e "s|content=\"A full-stack Leptos starter for Cloudflare Workers with D1-backed todos.\"|content=\"$PROJECT\"|" \
  "$ROOT_DIR/src/app.rs"

# --- Strip todo domain, replace with scaffold ---

log "Overwriting src/api.rs..."
cat > "$ROOT_DIR/src/api.rs" <<'RUST'
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

// Define your shared types and #[server] functions here.
// See docs/building-features.md for the full pattern.
RUST

log "Deleting src/server/todos.rs..."
rm "$ROOT_DIR/src/server/todos.rs"

log "Overwriting src/server/mod.rs..."
cat > "$ROOT_DIR/src/server/mod.rs" <<'RUST'
pub mod state;

pub use state::AppState;

use leptos::prelude::ServerFnError;

pub fn server_error(error: impl std::fmt::Display) -> ServerFnError {
    ServerFnError::ServerError(error.to_string())
}
RUST

log "Creating src/components/home_page.rs..."
cat > "$ROOT_DIR/src/components/home_page.rs" <<'RUST'
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <main class="page-shell">
            <section class="hero">
                <div class="hero-copy">
                    <h1>"It works."</h1>
                    <p class="hero-lede">
                        "Your Leptos app is running on Cloudflare Workers. "
                        "Edit this component in src/components/home_page.rs."
                    </p>
                </div>
            </section>
        </main>
    }
}
RUST

log "Deleting src/components/todo_page.rs..."
rm "$ROOT_DIR/src/components/todo_page.rs"

log "Overwriting src/components/mod.rs..."
printf 'pub mod home_page;\n' > "$ROOT_DIR/src/components/mod.rs"

log "Updating src/app.rs imports and route..."
sedi \
  -e 's|use crate::components::todo_page::TodoPage;|use crate::components::home_page::HomePage;|' \
  -e 's|view=TodoPage|view=HomePage|' \
  "$ROOT_DIR/src/app.rs"

log "Deleting migrations/0001_init.sql..."
rm "$ROOT_DIR/migrations/0001_init.sql"

log "Stripping todo styles from style/main.css..."
cat > "$ROOT_DIR/style/main.css" <<'CSS'
:root {
  --bg: #f4efe5;
  --bg-panel: rgba(255, 251, 245, 0.82);
  --bg-panel-strong: #fffaf2;
  --ink: #231c17;
  --ink-soft: #6f665d;
  --line: rgba(35, 28, 23, 0.12);
  --accent: #df5d2f;
  --accent-deep: #a73f1b;
  --accent-soft: rgba(223, 93, 47, 0.12);
  --success: #1f7a52;
  --shadow: 0 24px 80px rgba(61, 38, 16, 0.12);
  --radius-xl: 28px;
  --radius-lg: 20px;
  --radius-md: 14px;
  --radius-pill: 999px;
  --max-width: 1120px;
}

* {
  box-sizing: border-box;
}

html {
  background:
    radial-gradient(circle at top left, rgba(223, 93, 47, 0.24), transparent 32%),
    radial-gradient(circle at 85% 10%, rgba(31, 122, 82, 0.18), transparent 24%),
    linear-gradient(180deg, #f6f1e7 0%, #f0e8dc 100%);
  color: var(--ink);
  font-family: "Iowan Old Style", "Palatino Linotype", "Book Antiqua", Georgia, serif;
}

body {
  margin: 0;
  min-height: 100vh;
}

button,
input {
  font: inherit;
}

.page-shell {
  max-width: var(--max-width);
  margin: 0 auto;
  padding: 40px 20px 72px;
}

.hero {
  margin-bottom: 26px;
}

.eyebrow {
  display: inline-flex;
  margin: 0 0 18px;
  padding: 8px 14px;
  border: 1px solid rgba(35, 28, 23, 0.08);
  border-radius: var(--radius-pill);
  background: rgba(255, 255, 255, 0.58);
  color: var(--ink-soft);
  letter-spacing: 0.08em;
  text-transform: uppercase;
  font-family: "Avenir Next Condensed", "Franklin Gothic Medium", sans-serif;
  font-size: 0.78rem;
}

.hero-grid {
  display: grid;
  grid-template-columns: minmax(0, 1.35fr) minmax(320px, 0.95fr);
  gap: 22px;
  align-items: stretch;
}

.hero-copy,
.panel,
.feedback {
  border: 1px solid var(--line);
  border-radius: var(--radius-xl);
  background: var(--bg-panel);
  backdrop-filter: blur(18px);
  box-shadow: var(--shadow);
}

.hero-copy {
  padding: 34px;
}

.hero-copy h1 {
  margin: 0;
  font-size: clamp(2.7rem, 4vw, 4.7rem);
  line-height: 0.96;
  letter-spacing: -0.04em;
}

.hero-lede {
  max-width: 42rem;
  margin: 18px 0 0;
  color: var(--ink-soft);
  font-size: 1.08rem;
  line-height: 1.65;
}

.feedback {
  margin-bottom: 18px;
  padding: 16px 18px;
}

.feedback--error {
  border-color: rgba(167, 63, 27, 0.18);
  background: rgba(255, 240, 235, 0.85);
  color: var(--accent-deep);
}

.panel {
  padding: 24px;
}

.loading-panel {
  display: grid;
  gap: 14px;
  text-align: center;
}

.skeleton {
  border-radius: 14px;
  background:
    linear-gradient(
      100deg,
      rgba(255, 255, 255, 0.32) 20%,
      rgba(255, 255, 255, 0.7) 40%,
      rgba(255, 255, 255, 0.32) 60%
    )
    rgba(35, 28, 23, 0.08);
  background-size: 200% 100%;
  animation: shimmer 1.4s infinite linear;
}

.skeleton--title {
  height: 28px;
  width: 38%;
}

.skeleton--row {
  height: 72px;
}

.route-miss {
  padding: 64px 20px;
  text-align: center;
}

@keyframes shimmer {
  from {
    background-position: 200% 0;
  }

  to {
    background-position: -200% 0;
  }
}

@media (max-width: 860px) {
  .hero-grid {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 640px) {
  .page-shell {
    padding: 20px 14px 44px;
  }

  .hero-copy,
  .panel {
    padding: 20px;
    border-radius: 22px;
  }
}
CSS

# --- Next steps ---

cat <<EOF

[init] Done. "$PROJECT" is ready.

Next steps:
1. bunx wrangler d1 create $PROJECT-db
2. Replace the placeholder database IDs in wrangler.toml
3. Add a migrations/0001_init.sql for your schema
4. bunx wrangler d1 migrations apply $PROJECT-db --local
5. bash ./scripts/build-edge.sh
6. bunx wrangler dev --local --ip 127.0.0.1 --port 57581
EOF
