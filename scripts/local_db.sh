#!/bin/bash

# Check if argument is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <up or down>"
    exit 1
fi

UP_OR_DOWN="$1"

# Check if a custom user has been set, otherwise default to 'surreal'
export SURREAL_USER="${SURREAL_USER:=surreal}"
# Check if a custom password has been set, otherwise default to 'surreal'
export SURREAL_PASS="${SURREAL_PASS:=surreal}"

case $UP_OR_DOWN in
    "up")
        docker compose -f ./infrastructure/local/docker-compose.yml up -d 
        ;;
    "down")
        echo "Running staging scripts..."
        docker compose -f ./infrastructure/local/docker-compose.yml down
        ;;
    *)
        echo "Unsupported option: $ENV"
        exit 1
        ;;
esac
