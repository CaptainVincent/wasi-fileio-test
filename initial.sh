#!/bin/bash

folder_name="readonlyFolder"

# Remove old test folder
sudo rm -rf "$folder_name"

# Create the folder
mkdir "$folder_name"

# Create sub folder
mkdir "$folder_name/subFolder"
mkdir "$folder_name/subFolder2"

# Set the permissions to read-only
chmod 555 "$folder_name"
