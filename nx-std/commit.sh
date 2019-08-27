#!/bin/bash
for file in $(git diff --name-only); do
	git add $file && git commit -m "$file - $(date -R)"
done
