set shell := ["bash", "-uc"]

default:
  @just --choose --justfile {{justfile()}}

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

