#!/bin/bash

# run command with exec to pass control
echo "Running CMD: $@"
exec /opt/scripts/wait-for-it.sh back:3001 --timeout=0 -- "$@"
