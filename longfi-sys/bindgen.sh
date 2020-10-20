#!/bin/sh

bindgen \
    src/bindings.h \
    -o src/bindings.rs \
    --whitelist-function '^lfc_.*' \
    --whitelist-var '^lfc_.*' \
    -- -I vendor/include
