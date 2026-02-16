#!/bin/bash
echo "Checking for changes in packages/website and packages/theme..."
git diff HEAD^ HEAD --quiet packages/website/ packages/theme/
