#!/bin/bash

set -e

echo -e "\033[36;1mRunning library tests:\033[0m"
(cd config_struct && cargo test --all-features)

echo -e "\033[36;1mRunning integration tests:\033[0m"
(cd config_struct_test && cargo test)

echo -e "\033[36;1mRunning release-mode integration tests:\033[0m"
(cd config_struct_test && cargo test --release)
