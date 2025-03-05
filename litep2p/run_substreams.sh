#!/bin/bash

declare -A results

for index in 1 32 64 128 256; do
    echo "Running test with $index substreams..."

    # Start the server
    RUST_LOG=info cargo run -- server --listen-address "/ip6/::/tcp/33333" --node-key "secret" > /dev/null 2>&1 &

    # Get the PID of the server
    SERVER_PID=$!

    # Start the client and capture the output
    OUTPUT=$(RUST_LOG=info cargo run -- client-substream --server-address "/ip6/::1/tcp/33333/p2p/12D3KooWBpZHDZu7YSbvPaPXKhkRNJvR7MkTJMQQAVBKx9mCqz3q" --substreams $index | grep "Average time to open substreams")

    # Store the result in an associative array
    results[$index]="$OUTPUT"

    # Kill the server
    kill $SERVER_PID
done

# Print sorted results
echo "Results Summary:"
for index in $(echo "${!results[@]}" | tr ' ' '\n' | sort -n); do
    echo "Substreams: $index -> ${results[$index]}"
done

echo
echo
echo

# Header for Markdown table
echo "# Test Results"
echo
echo "| Substreams | Average Time to Open Substreams |"
echo "|------------|--------------------------------|"
# Print sorted results and format them as a Markdown table
for index in $(echo "${!results[@]}" | tr ' ' '\n' | sort -n); do
    avg=$(echo ${results[$index]} | awk '{print $11}' | cut -d '=' -f 2)
    echo "| $index        | ${avg} |"
done
