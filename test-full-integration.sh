#!/bin/bash

# R5 Flowlight - Full Integration Test
# Tests daemon + overlay together

echo "🚀 R5 Flowlight - Full Integration Test"
echo "========================================"

echo ""
echo "🧪 Starting daemon and overlay in parallel..."
echo ""

# Start daemon in background
echo "🔥 Starting search daemon..."
cargo run --bin simple-daemon -- --log-level info &
DAEMON_PID=$!

# Wait for daemon to start
sleep 3

# Start overlay in background  
echo "🪟 Starting search overlay..."
cargo run --bin simple-overlay -- --log-level info &
OVERLAY_PID=$!

# Wait for overlay to start
sleep 2

echo ""
echo "✅ Both processes started successfully!"
echo "🔥 Daemon PID: $DAEMON_PID"
echo "🪟 Overlay PID: $OVERLAY_PID"
echo ""
echo "🏃 Integration test running..."
echo "💡 You should see both daemon and overlay logs interleaved"
echo "⌨️  Global shortcuts simulated every 10s"
echo "🔍 Search queries simulated every 3s when overlay visible"
echo ""
echo "Press Ctrl+C to stop both processes"
echo ""

# Function to cleanup on exit
cleanup() {
    echo ""
    echo "🛑 Stopping integration test..."
    kill $DAEMON_PID 2>/dev/null
    kill $OVERLAY_PID 2>/dev/null
    echo "✅ Integration test stopped"
    exit 0
}

# Set trap to cleanup on Ctrl+C
trap cleanup INT

# Wait for processes
wait $DAEMON_PID $OVERLAY_PID