#~ /bin/bash
# badprog.com

# First param
COMMAND=${1-run}

echo "Command: $COMMAND"

# 
for pkg in \
    $(cargo metadata --format-version=1 | \
        jq -r '.workspace_members[] | split("/") | last | split("#") | first'); \
    do
    echo "--| Sub-project: $pkg"
    cargo $COMMAND -p "$pkg"
done
