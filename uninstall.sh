#!/bin/bash

target=$(which jchess)

if [[ ${target} == "" ]]; then
    echo "Could not find jchess in \$PATH"
    exit
fi

echo "Deleting ${target}"
rm ${target}

