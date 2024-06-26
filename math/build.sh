#!/bin/bash

# Clean previous builds
cargo clean

# Build the Rust library for aarch64
cargo ndk --target aarch64-linux-android -o build/lib/aarch64 build --release

# Build the Rust library for x86_64
cargo ndk --target x86_64-linux-android -o build/lib/x86_64 build --release

# Create necessary directories for AAR structure
mkdir -p build/aar/jniLibs/arm64-v8a build/aar/jniLibs/x86_64 build/aar/src/main/kotlin/com/example/math

# Move the generated .so files to the appropriate location
cp build/lib/aarch64/aarch64-linux-android/release/libmath_lib.so build/aar/jniLibs/arm64-v8a/
cp build/lib/x86_64/x86_64-linux-android/release/libmath_lib.so build/aar/jniLibs/x86_64/

# Generate the Kotlin file
echo "package com.example.math

class MathLib {
    external fun add(a: Int, b: Int): Int

    companion object {
        init {
            System.loadLibrary(\"math_lib\")
        }
    }
}" > build/aar/src/main/kotlin/com/example/math/MathLib.kt

# Create the AAR file
cd build/aar
zip -r math_lib.aar *

# Move the AAR file to the project root
mv math_lib.aar ../../
