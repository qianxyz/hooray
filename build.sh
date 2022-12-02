#!/usr/bin/env bash

set -xe

cargo run --release > image.ppm
display image.ppm
