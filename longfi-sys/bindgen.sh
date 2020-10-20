#!/bin/sh

bindgen \
    src/bindings.h \
    --output src/bindings.rs \
    --whitelist-function '^lfc_.*' \
    --whitelist-var '^lfc_.*' \
    -- -I vendor/include
