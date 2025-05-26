#!/bin/bash

path=/

if [[ $1 != "" ]]; then
    path=$1
fi

cargo install --path ./ --root $path

