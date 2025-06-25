#!/bin/bash
#
# Make a bunch of commits so we can test git-quest
# Usage: ./makecommits.sh <n_commits>

# Check if argument is provided
if [ $# -eq 0 ]; then
    echo "Usage: $0 <n_fortunes>"
    echo "Example: $0 5"
    exit 1
fi

n_fortunes=$1

# Check if argument is a number
if ! [[ "$n_fortunes" =~ ^[0-9]+$ ]]; then
    echo "Error: n_fortunes must be a positive integer"
    exit 1
fi

# Check if fortune command exists
if ! command -v fortune &> /dev/null; then
    echo "Error: fortune command not found. Please install fortune-mod package"
    exit 1
fi

# Loop n_fortunes times
for ((i=1; i<=n_fortunes; i++)); do
    echo "Adding fortunes $i of $n_fortunes..."

    # Get a fortune and append to fortunes.txt
    fortune >> fortunes.txt

    # Add an empty line between fortunes for readability
    echo "" >> fortunes.txt

    # Count lines in fortunes.txt
    line_count=$(wc -l < fortunes.txt)

done
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
echo "Successfully created $n_fortunes fortunes!"
