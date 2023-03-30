#!/bin/bash

# not sure why this is, but snap isn't running correctly

if command -v insomnia &> /dev/null
then
    echo "running via command..."
    insomnia
else
    echo "running via snap..."
    sudo -u msti snap run insomnia
fi

