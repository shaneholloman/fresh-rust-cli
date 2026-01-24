#!/bin/bash
set -e

# Clean
rm -rf dist

# Build VitePress docs
# It will build to dist/docs because of outDir in config.ts
bun run docs:build

# Copy custom homepage to root of dist
cp index.html dist/

# Copy homepage assets to dist/assets
mkdir -p dist/assets
if [ -d "public/assets" ] && [ "$(ls -A public/assets)" ]; then
    cp -r public/assets/* dist/assets/
fi

# Add .nojekyll to bypass Jekyll on GitHub Pages
touch dist/.nojekyll

echo "Build complete! Output is in dist/"
