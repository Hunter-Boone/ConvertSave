#!/bin/bash

# Suppress dconf and graphics warnings on Linux
export DCONF_PROFILE=/dev/null
export GDK_BACKEND=x11
export MESA_LOADER_DRIVER_OVERRIDE=i965

# Filter out specific warning patterns while preserving other output
tauri dev 2>&1 | grep -v -E "(dconf-WARNING|libEGL warning|failed to open.*renderD128)" || true 