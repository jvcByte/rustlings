#!/bin/bash

# AI Commit Message Generator
# Analyzes staged git changes and generates a commit message using Ollama (free, local)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if Ollama is installed
if ! command -v ollama &> /dev/null; then
    echo -e "${RED}Error: Ollama is not installed${NC}"
    echo -e "${YELLOW}Install it from: https://ollama.ai${NC}"
    echo -e "${YELLOW}Then run: ollama pull llama3.2${NC}"
    exit 1
fi

# Check if Ollama is running
if ! curl -s http://localhost:11434/api/tags &> /dev/null; then
    echo -e "${RED}Error: Ollama is not running${NC}"
    echo -e "${YELLOW}Start it with: ollama serve${NC}"
    exit 1
fi

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

# Limit diff size to avoid overwhelming the model
DIFF_SIZE=${#DIFF}
if [ $DIFF_SIZE -gt 4000 ]; then
    DIFF=$(echo "$DIFF" | head -c 4000)
    echo -e "${YELLOW}Note: Large diff truncated for analysis${NC}"
fi

# Prepare the prompt
PROMPT="Analyze this git diff and generate a single line commit message following conventional commit format.

Format: <type>: <description>

Types: feat, fix, docs, style, refactor, test, chore

Rules:
- Keep description under 72 characters
- Use present tense (\"add\" not \"added\")
- Don't capitalize first letter of description
- No period at the end
- Be specific about what changed
- Output ONLY the commit message, nothing else

Git diff:
${DIFF}

Commit message:"

# Make API request to Ollama
echo -e "${GREEN}Generating commit message...${NC}"

RESPONSE=$(curl -s http://localhost:11434/api/generate -d "{
  \"model\": \"llama3.2\",
  \"prompt\": $(echo "$PROMPT" | jq -Rs .),
  \"stream\": false
}")

# Extract the commit message from response
COMMIT_MSG=$(echo "$RESPONSE" | jq -r '.response' | head -n 1 | sed 's/^[[:space:]]*//' | sed 's/[[:space:]]*$//')

if [ -z "$COMMIT_MSG" ] || [ "$COMMIT_MSG" = "null" ]; then
    echo -e "${RED}Error: Failed to generate commit message${NC}"
    echo "Response: $RESPONSE"
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
