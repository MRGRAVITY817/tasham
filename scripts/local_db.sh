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

export SURREAL_URL="127.0.0.1:8080"

case $UP_OR_DOWN in
    "up")
        docker compose -f ./infrastructure/local/docker-compose.yml up -d 
        RUSTFLAGS="-Zthreads=8" cargo +nightly watch -x run
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
