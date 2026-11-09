# NOTICE

**Airgorah-RT** is a fork of [**Airgorah**](https://github.com/martin-olivier/airgorah),
created and maintained by **Martin Olivier** (martin.olivier@live.fr).

All of the original design, architecture, and the large majority of the
code in this repository (the scanning, deauthentication, handshake/PMKID
capture, cracking, and privileged-agent logic, and the base GTK4
interface) are Martin Olivier's work, released under the MIT License — see
[`LICENSE`](LICENSE).

## What this fork changes

This fork, maintained by Syed Muhammad Tayyab, adds on top of upstream:

- An **Appearance setting** (System / Light / Dark theme), persisted in the
  user's config and applied live — see `crates/gui/src/frontend/interfaces/settings.rs`,
  `crates/gui/src/frontend/connections/settings.rs`, and the `theme` field
  on `Settings` in `crates/common/src/types.rs`.
- **Additional packaging**: a Flatpak manifest under `packaging/flatpak/`,
  and a native (non-Docker) build script at `scripts/build-native.sh`,
  alongside the original Docker/`fpm` pipeline.

No part of the upstream codebase's authorship is claimed as original work
by this fork; see `git log` / `git blame` for the full history and the
[upstream repository](https://github.com/martin-olivier/airgorah) for the
canonical project.

## Why fork instead of contributing upstream?

These changes started as a learning exercise while studying Airgorah's
codebase during a Red Team internship. Where appropriate, changes made
here may also be proposed upstream via a pull request.
