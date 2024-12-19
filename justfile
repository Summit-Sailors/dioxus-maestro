set shell := ["bash", "-uc"]
set export
set dotenv-load

default:
  @just --choose --justfile {{justfile()}}

help:
  #!/usr/bin/env bash
  set -euo pipefail
  echo DEFAULT:
  just --list

web:
  #!/usr/bin/env bash
  set -euo pipefail
  dx serve --platform web -p upwork-app

desktop:
  #!/usr/bin/env bash
  set -euo pipefail
  dx serve --platform desktop -p upwork-app

mobile:
  #!/usr/bin/env bash
  set -euo pipefail
  dx serve --platform mobile -p upwork-app

tailwind:
  #!/usr/bin/env bash
  set -euo pipefail
  cd ./crates/upwork-app/
  npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch

task-server:
  #!/usr/bin/env bash
  set -euo pipefail
  cargo run -p upwork-tasks --features task-server

chrome-fastapi:
  #!/usr/bin/env bash
  set -euo pipefail
  rye run python upwork-scrape/upwork_scrape/server.py

install-sys-deps:
  #!/usr/bin/env bash
  set -euo pipefail
  sudo dnf install -y postgresql-devel \
    postgresql-server \
    postgresql-contrib \
    gtk3-devel \
    gdk-pixbuf2-devel \
    pango-devel \
    atk-devel \
    webkit2gtk4.1-devel \
    cairo-gobject-devel \
    libxdo-devel \
    chromium \
    chromium-headless \
    chromedriver \
    xorg-x11-server-devel \
    libX11-devel \
    libXi-devel \
    libXrandr-devel \
    libXinerama-devel \
    libXcursor-devel

clear:
  #!/usr/bin/env bash
  set -euo pipefail
  rm -rf ~/.cargo/.package-cache ~/.cargo/registry ~/.cache/rust-analyzer *.lock target .venv
  rye sync

migration-run:
  #!/usr/bin/env bash
  set -euo pipefail
  diesel migration run --config-file crates/upwork-db/diesel.toml

migration-gen name:
  #!/usr/bin/env bash
  set -euo pipefail
  diesel migration generate --migration-dir crates/upwork-db/migrations "$name"