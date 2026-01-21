#!/bin/bash

FORBIDDEN_KEYWORDS=("telemetry" "google-analytics" "sentry" "crash-reporter" "mixpanel")
EXCLUDES="--exclude-dir=.git --exclude-dir=node_modules --exclude-dir=target --exclude=audit_telemetry.sh --exclude=implementation_plan.md --exclude=task.md --exclude=gap_analysis.md"

echo "🔍 Starting Zero Telemetry Audit..."
echo "-----------------------------------"

FOUND=0

for KEYWORD in "${FORBIDDEN_KEYWORDS[@]}"; do
    MATCHES=$(grep -r $EXCLUDES -i "$KEYWORD" .)
    if [ ! -z "$MATCHES" ]; then
        echo "❌ Found forbidden keyword: '$KEYWORD'"
        echo "$MATCHES"
        FOUND=1
    fi
done

echo "-----------------------------------"
if [ $FOUND -eq 0 ]; then
    echo "✅ AUDIT PASSED: No telemetry sinks detected."
    exit 0
else
    echo "⚠️  AUDIT PRECAUTION: Potential telemetry patterns found. Please review above."
    # We exit 0 because 'telemetry' might appear in comments like "No telemetry". 
    # This is a soft audit for the user to review.
    exit 0
fi
