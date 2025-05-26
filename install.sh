#!/bin/bash

path=/bin

if [[ $1 != "" ]]; then
    path=$1
fi

cargo build

echo "Installing to $path/"
mv target/debug/jchess $path/
