#!/usr/bin/env bash
docker build -t breakpoint:latest .
docker build -f Dockerfile.prebuilt -t breakpoint:latest-prebuilt .
