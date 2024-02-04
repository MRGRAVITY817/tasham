#!/bin/bash

# Check if ENV argument is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <ENV>"
    exit 1
fi

ENV="$1"

# Run scripts based on the deployment environment
case $ENV in
    "development")
        echo "Running development scripts..."
        # Add commands to run for the development environment
        # For example:
        # ./development_script.sh
        ;;
    "production")
        echo "Running production scripts..."
        # Add commands to run for the production environment
        # For example:
        # ./production_script.sh
        ;;
    *)
        echo "Unsupported environment: $ENV"
        exit 1
        ;;
esac

echo "Script execution for environment '$ENV' completed."
