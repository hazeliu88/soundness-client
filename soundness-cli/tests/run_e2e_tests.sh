#!/bin/bash

# Start the server in the background
cd ../soundness-server
cargo run &
SERVER_PID=$!

# Wait for the server to start
sleep 2

# Run the tests
cd ../soundness-cli
cargo test --test e2e_test -- --nocapture

# Kill the server
kill $SERVER_PID 