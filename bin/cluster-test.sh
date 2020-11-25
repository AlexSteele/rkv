#!/usr/bin/env bash

echo "starting services"
cargo build
target/debug/rkv --address 127.0.0.1:8078 &
target/debug/rkv --address 127.0.0.1:8079 &
target/debug/rkv --address 127.0.0.1:8080 --seed-nodes "127.0.0.1:8078","127.0.0.1:8079" &

echo "waiting for startup"
sleep 5

echo "running test"
cargo test test_rkv

echo "killing services"
trap 'kill $(jobs -p)' EXIT
