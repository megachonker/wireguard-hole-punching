#!/bin/bash
VPS=$(getent hosts vps| awk '{print $1}')
cargo rustc -r -- -C target-feature=+crt-static
scp target/release/wireguard-hole-punching vps:wireguard-hole-punching

# Create a new tmux session
tmux new-session -d -s MySession

# VPS
tmux send-keys -t MySession 'ssh vps -- timeout 10 ./wireguard-hole-punching -r' C-m


# Local server
tmux split-window -v -t MySession
tmux send-keys -t MySession "sleep 1 && target/release/wireguard-hole-punching -s $VPS" C-m

# local Client
tmux split-window -v -t MySession
tmux send-keys -t MySession "sleep 1 && target/release/wireguard-hole-punching -c $VPS" C-m

# Attach to the tmux session
tmux attach -t MySession

echo "All commands launched successfully."
