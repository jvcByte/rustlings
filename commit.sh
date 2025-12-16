#!/bin/bash

# AI Commit Message Generator
# Analyzes staged git changes and generates a commit message using Claude API

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo -e "${RED}Error: Not a git repository${NC}"
    exit 1
fi

# Check if there are staged changes
if git diff --cached --quiet; then
    echo -e "${YELLOW}No staged changes found. Stage your changes with 'git add' first.${NC}"
    exit 1
fi

# Get the diff of staged changes
echo -e "${GREEN}Analyzing staged changes...${NC}"
DIFF=$(git diff --cached)

# Prepare the API request
read -r -d '' PROMPT << EOM || true
Analyze this git diff and generate a concise, informative commit message following conventional commit format.

Format: <type>: <description>

Types: feat, fix, docs, style, refactor, test, chore

Rules:
- Keep description under 72 characters
- Use present tense ("add" not "added")
- Don't capitalize first letter of description
- No period at the end
- Be specific about what changed

Git diff:
${DIFF}
EOM

# Make API request to Claude
echo -e "${GREEN}Generating commit message...${NC}"

RESPONSE=$(curl -s https://api.anthropic.com/v1/messages \
  -H "Content-Type: application/json" \
  -d @- << EOF
{
  "model": "claude-sonnet-4-20250514",
  "max_tokens": 1000,
  "messages": [
    {
      "role": "user",
      "content": $(echo "$PROMPT" | jq -Rs .)
    }
  ]
}
EOF
)

# Extract the commit message from response
COMMIT_MSG=$(echo "$RESPONSE" | jq -r '.content[0].text' | head -n 1)

if [ -z "$COMMIT_MSG" ] || [ "$COMMIT_MSG" = "null" ]; then
    echo -e "${RED}Error: Failed to generate commit message${NC}"
    echo "API Response: $RESPONSE"
    exit 1
fi

echo -e "\n${GREEN}Generated commit message:${NC}"
echo -e "${YELLOW}${COMMIT_MSG}${NC}\n"

# Ask user if they want to use this message
read -p "Use this commit message? (y/n/e for edit): " -n 1 -r
echo

if [[ $REPLY =~ ^[Yy]$ ]]; then
    git commit -m "$COMMIT_MSG"
    echo -e "${GREEN}✓ Committed successfully!${NC}"
elif [[ $REPLY =~ ^[Ee]$ ]]; then
    # Open git commit in editor with the generated message
    git commit -e -m "$COMMIT_MSG"
else
    echo -e "${YELLOW}Commit cancelled.${NC}"
    exit 0
fi
