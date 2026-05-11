#!/usr/bin/env bash
# Mirror reproducibility artifacts (parquets + figures + paper) to a public S3
# bucket. SPEC §10 asks for `s3://dynamis-memory-geometry/`.
#
# Prereqs (run once):
#   aws configure           # enter AWS keys
#   aws s3 mb s3://dynamis-memory-geometry --region us-west-2
#   aws s3api put-bucket-policy --bucket dynamis-memory-geometry \
#     --policy file://scripts/s3_public_read_policy.json
#
# Usage:
#   bash scripts/mirror_to_s3.sh
#
set -euo pipefail
BUCKET="${GAC_S3_BUCKET:-dynamis-memory-geometry}"
PREFIX="${GAC_S3_PREFIX:-v1.0}"
REGION="${AWS_REGION:-us-west-2}"

echo "[mirror] bucket=s3://$BUCKET/$PREFIX (region=$REGION)"

# 1. Parquets (per-experiment results) -- small, always sync
aws s3 sync results/ "s3://$BUCKET/$PREFIX/results/" \
  --exclude "*.jsonl" --exclude "*partial*" \
  --content-type application/octet-stream

# 2. Paper artifacts (PDFs + figures + tables + tex sources)
aws s3 cp paper/main.pdf "s3://$BUCKET/$PREFIX/paper/main.pdf"
aws s3 cp paper/supp.pdf "s3://$BUCKET/$PREFIX/paper/supp.pdf"
aws s3 sync paper/figs/ "s3://$BUCKET/$PREFIX/paper/figs/" --content-type application/pdf
aws s3 sync paper/tables/ "s3://$BUCKET/$PREFIX/paper/tables/" --content-type text/x-tex
aws s3 cp paper/main.tex "s3://$BUCKET/$PREFIX/paper/main.tex"
aws s3 cp paper/supp.tex "s3://$BUCKET/$PREFIX/paper/supp.tex"
aws s3 cp paper/refs.bib "s3://$BUCKET/$PREFIX/paper/refs.bib" || true

# 3. SPEC + README + reproduction guide
aws s3 cp SPEC.md "s3://$BUCKET/$PREFIX/SPEC.md"
aws s3 cp README.md "s3://$BUCKET/$PREFIX/README.md"
aws s3 cp REPRODUCE.md "s3://$BUCKET/$PREFIX/REPRODUCE.md"
aws s3 cp SPEC_AUDIT_FINAL.md "s3://$BUCKET/$PREFIX/SPEC_AUDIT_FINAL.md"
aws s3 cp CITATION.cff "s3://$BUCKET/$PREFIX/CITATION.cff"

# 4. Version manifest
MANIFEST=$(mktemp)
cat > "$MANIFEST" <<EOF
{
  "version": "$PREFIX",
  "git_sha": "$(git rev-parse HEAD)",
  "uploaded_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "bucket": "$BUCKET",
  "sizes": {
$(cd results && for f in */*.parquet; do echo "    \"$f\": $(stat -c %s "$f"),"; done | sed '$s/,$//')
  }
}
EOF
aws s3 cp "$MANIFEST" "s3://$BUCKET/$PREFIX/manifest.json" --content-type application/json

echo "[mirror] DONE -- browse at https://$BUCKET.s3.$REGION.amazonaws.com/$PREFIX/"
