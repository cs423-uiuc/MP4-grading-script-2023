#!/bin/bash

cd ebpf_xdp_blocker || exit 1
./blocker 1 & # run in background
# pid=$!
sleep 2

echo "====== block port 8081 udp"
./blocker add_rule 8081 udp
../port_test

echo "====== block port 8081 tcp"
./blocker add_rule 8081 tcp
../port_test

# kill $pid
# sleep 1
