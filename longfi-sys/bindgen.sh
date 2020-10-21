#!/bin/sh

bindgen \
    src/bindings.h \
    --no-copy '^lfc.*' \
    --output src/bindings.rs \
    --use-core \
    --whitelist-function '^lfc_.*' \
    --whitelist-var '^lfc_.*' \
    --rustified-enum '^lfc_res' \
    --whitelist-function '^cursor_.*' \
    -- \
    -I vendor/include \
    -I vendor/extra/cursor/include
