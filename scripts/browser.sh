#!/bin/bash

if [ -z "$CLIENT_URL" ]
then
    echo "CLIENT_URL environment variable is not set. Exiting."
    exit 1
fi

firefox --new-window "$CLIENT_URL"
