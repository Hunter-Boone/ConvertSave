#!/bin/bash

# Suppress dconf and graphics warnings on Linux
export DCONF_PROFILE=/dev/null
export GDK_BACKEND=x11
export MESA_LOADER_DRIVER_OVERRIDE=i965
export NO_AT_BRIDGE=1

# Filter out all known warning patterns while preserving important output
tauri dev 2>&1 | grep -v -E "(dconf-WARNING|libEGL warning|failed to open.*renderD128|Gdk-WARNING|Tried to unmap the parent of a popup|Could not connect: No such file or directory)" || true 