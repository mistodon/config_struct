#!/bin/bash

set -e

(cd config_struct && cargo test --all-features)
(cd config_struct_test && cargo test)
