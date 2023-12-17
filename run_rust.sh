#!/bin/bash

cd ./samples/mp4-sample || exit 1
./entry 1 & # run in background
# pid=$!
sleep 2

echo "====== block port 8081 tcp"
./entry add_rule 8081 tcp
../../port_test

echo "====== block port 8081 udp"
./entry add_rule 8081 udp
../../port_test

# kill $pid
# sleep 1
