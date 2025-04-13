#!/bin/bash

# Build script for timezone databases
# Usage: ./build_tzdb.sh 2018c

if [ -z "$1" ]; then
  echo "Usage: ./build_tzdb.sh <tzversion> (e.g., 2018c)"
  exit 1
fi

VERSION=$1
TZCODE="tzcode${VERSION}.tar.gz"
TZDATA="tzdata${VERSION}.tar.gz"
WORKDIR="/tmp/tzwork_$$"
PROJECTROOT=$(pwd)
DESTDIR="${PROJECTROOT}/data/timezones/zoneinfo-${VERSION}"

# Create working directory
mkdir -p "$WORKDIR"
cd "$WORKDIR" || exit 1

# Download tzcode and tzdata directly to the working directory
echo "Downloading ${TZCODE}..."
curl -O "https://data.iana.org/time-zones/releases/${TZCODE}"

echo "Downloading ${TZDATA}..."
curl -O "https://data.iana.org/time-zones/releases/${TZDATA}"

# Extract tzcode and tzdata
tar -xzf "${TZCODE}"
tar -xzf "${TZDATA}"

# Build and install
echo "Building timezone database version ${VERSION}..."
make
make TOPDIR="$DESTDIR" install

# Clean up
cd "$PROJECTROOT"
rm -rf "$WORKDIR"

echo "Timezone database version ${VERSION} installed to $DESTDIR"
