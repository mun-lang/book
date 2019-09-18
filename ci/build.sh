#!/bin/bash

set -e

export PATH=$PATH:/home/travis/.cargo/bin;

echo 'Building...'
mdbook build