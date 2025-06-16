#!/bin/bash
echo "Testing search-daemon build..."
cd /Users/rafaelreis/R5\ Flowlight
cargo check -p search-daemon --bin r5-flowlight-daemon --offline 2>&1 | head -10