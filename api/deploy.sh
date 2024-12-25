#!/bin/bash

# Define variables
BUILD_DIR="./target/lambda/release"
DEPLOY_DIR="./.aws-sam/artifacts"
TIMESTAMP=$(date +%s)
ARTIFACT_PATH="$DEPLOY_DIR/lambda-$TIMESTAMP"

# Ensure the deployment directory exists
mkdir -p $DEPLOY_DIR

# Build the project
echo "Building the project..."
cargo build --release --target x86_64-unknown-linux-musl

# Copy the built artifact to a versioned path
echo "Preparing deployment artifact..."
cp -r $BUILD_DIR $ARTIFACT_PATH

# Update the CodeUri in template.yaml
echo "Updating CodeUri in template.yaml..."
sed -i.bak "s|CodeUri:.*|CodeUri: $ARTIFACT_PATH|" template.yaml

# Deploy the stack
echo "Deploying the stack..."
sam deploy --guided
