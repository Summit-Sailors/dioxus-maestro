set shell := ["bash", "-uc"]
set export
set dotenv-load


DERIVE_ORDER:="Debug,Default,Error,Clone,Copy,Deref,DerefMut,PartialEq,Eq,PartialOrd,Ord,Hash,Serialize,Deserialize,FromStr,EnumIter,Display,EnumString,JsonSchema"

default:
  @just --choose --justfile {{justfile()}}


sort-d:
  #!/usr/bin/env bash
  set -euo pipefail
  cargo sort-derives

chrome-fastapi:
  #!/usr/bin/env bash
  set -euo pipefail
  rye run python chrome-fastapi/chrome_fastapi/server.py

clear:
  #!/usr/bin/env bash
  set -euo pipefail
  cargo clean
  rm *.lock