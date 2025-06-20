#!/bin/bash
#
# Make a bunch of commits so we can test git-quest
# Usage: ./makecommits.sh <n_commits>

# Check if argument is provided
if [ $# -eq 0 ]; then
    echo "Usage: $0 <n_commits>"
    echo "Example: $0 5"
    exit 1
fi

n_commits=$1

# Check if argument is a number
if ! [[ "$n_commits" =~ ^[0-9]+$ ]]; then
    echo "Error: n_commits must be a positive integer"
    exit 1
fi

# Check if fortune command exists
if ! command -v fortune &> /dev/null; then
    echo "Error: fortune command not found. Please install fortune-mod package"
    exit 1
fi

echo "Making $n_commits commits..."

# Loop n_commits times
for ((i=1; i<=n_commits; i++)); do
    echo "Creating commit $i of $n_commits..."

    # Get a fortune and append to fortunes.txt
    fortune >> fortunes.txt

    # Add an empty line between fortunes for readability
    echo "" >> fortunes.txt

    # Count lines in fortunes.txt
    line_count=$(wc -l < fortunes.txt)

    # Add the file to git
    git add fortunes.txt

    # Make the commit
    git commit -m "commit to line $line_count"

    if [ $? -eq 0 ]; then
        echo "Commit $i completed successfully"
    else
        echo "Error: Failed to make commit $i"
        exit 1
    fi
done

echo "Successfully created $n_commits commits!"
