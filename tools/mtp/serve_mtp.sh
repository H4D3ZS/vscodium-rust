#!/bin/bash
# Reference MTP launch for Qwen3.8-27B, tuned from the community sweeps in
# github.com/sudoingX/qwen38-mtp. The kortex panel builds an equivalent argv
# via kortex_gac/launcher.rs (--spec-type draft-mtp forces --parallel 1); this
# script is for probing outside the IDE.
#
#   ./serve_mtp.sh /path/to/Qwen3.8-27B-UD-IQ4_XS.gguf [n_max] [p_min]
#
# RX 9060 XT 16 GB (the card kortex targets): the community row for this exact
# card runs AtomicChat's IQ3_XXS at n-max 2 -> ~29-31 tok/s. A less-packed
# 16 GB RDNA3 (RX 7900 GRE) at IQ3_XXS + tighter KV hit ~48 tok/s on live
# agent traffic with n-max 3, p-min 0.75. Start at n-max 2; sweep 2->4.

MODEL="${1:?path to a Qwen3.8-27B GGUF (the MTP head is baked in — unsloth / AtomicChat)}"
NMAX="${2:-2}"
PMIN="${3:-}"

ARGS=(
  -m "$MODEL"
  -c 131072 -ngl 999 -fa 1
  --cache-type-k q4_0 --cache-type-v q4_0
  --spec-type draft-mtp --spec-draft-n-max "$NMAX"
  --parallel 1
  --host 127.0.0.1 --port 8081
)
[ -n "$PMIN" ] && ARGS+=(--spec-draft-p-min "$PMIN")

exec llama-server "${ARGS[@]}"
