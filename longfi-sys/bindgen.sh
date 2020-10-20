#!/bin/sh

bindgen \
    src/bindings.h \
    --no-copy '^lfc.*' \
    --output src/bindings.rs \
    --use-core \
    --whitelist-function '^lfc_.*' \
    --whitelist-var '^lfc_.*' \
    -- -I vendor/include
