#!/bin/bash
# // https://github.com/badprog/badprog_leetcode_rust

# First param
# Change it with "test" in order to launch every test in all sub-projects.
# Otherwise it launches the run command on all sub-projects.
COMMAND=${1-run}

echo "Command: $COMMAND"

# 
for pkg in \
    $(cargo metadata --format-version=1 | \
        jq -r '.workspace_members[] | split("/") | last | split("#") | first'); \
    do
    echo "--| Sub-project: $pkg"
    cargo $COMMAND -p "$pkg" # launch the command with its param on each sub-project.
done
