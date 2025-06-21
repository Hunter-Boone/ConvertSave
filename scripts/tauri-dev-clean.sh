#!/bin/bash

# Suppress ALL Linux warnings for clean development
export DCONF_PROFILE=/dev/null
export GDK_BACKEND=x11
export MESA_LOADER_DRIVER_OVERRIDE=i965
export NO_AT_BRIDGE=1
export GTK_DEBUG=""

# Run tauri dev and suppress all stderr warnings (keeps stdout for important messages)
tauri dev 2>/dev/null 