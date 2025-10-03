#!/bin/bash

# Build script for wasim + Zola integration

echo "🦀 Building wasim WASM module..."
cd wasim
wasm-pack build --target web --out-dir pkg

if [ $? -ne 0 ]; then
    echo "❌ WASM build failed!"
    exit 1
fi

echo "📦 Copying WASM files to Zola static directory..."
cd ..
cp wasim/pkg/wasim.js site/static/
cp wasim/pkg/wasim_bg.wasm site/static/

echo "🏗️ Building Zola site..."
cd site
zola build

if [ $? -ne 0 ]; then
    echo "❌ Zola build failed!"
    exit 1
fi

echo "✅ Build complete! Site is ready in site/public/"
