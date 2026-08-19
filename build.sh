#!/bin/bash

# Build binaries and copy them to the out folder

docker buildx build --output type=local,dest=./out .