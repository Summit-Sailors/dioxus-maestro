set shell := ["bash", "-uc"]

default:
  @just --choose --justfile {{justfile()}}

clear:
  #!/usr/bin/env bash
  set -euo pipefail
  rm -rf ~/.cargo/.package-cache ~/.cargo/registry ~/.cache/rust-analyzer *.lock target .venv

sort-d:
  #!/usr/bin/env bash
  set -euo pipefail
  cargo sort-derives
  