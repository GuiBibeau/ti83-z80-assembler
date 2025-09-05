#!/bin/bash
# Comprehensive linting and quality check script for the Z80 Assembler

set -e

echo "==================================="
echo "Z80 Assembler Quality Check"
echo "==================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}Error: Cargo.toml not found. Run this script from the project root.${NC}"
    exit 1
fi

echo ""
echo "1. Checking code formatting..."
echo "-------------------------------"
if cargo fmt --check; then
    echo -e "${GREEN}✓ Code formatting is correct${NC}"
else
    echo -e "${RED}✗ Code needs formatting. Run 'cargo fmt' to fix.${NC}"
    exit 1
fi

echo ""
echo "2. Running Clippy lints..."
echo "-------------------------------"
if cargo clippy -- -D warnings 2>&1 | tee /tmp/clippy_output.txt; then
    echo -e "${GREEN}✓ No clippy warnings${NC}"
else
    echo -e "${RED}✗ Clippy found issues that need fixing${NC}"
    echo "See output above for details."
    exit 1
fi

echo ""
echo "3. Running tests..."
echo "-------------------------------"
if cargo test --quiet; then
    echo -e "${GREEN}✓ All tests passed${NC}"
else
    echo -e "${RED}✗ Some tests failed${NC}"
    exit 1
fi

echo ""
echo "4. Building documentation..."
echo "-------------------------------"
if cargo doc --no-deps --quiet; then
    echo -e "${GREEN}✓ Documentation builds successfully${NC}"
else
    echo -e "${YELLOW}⚠ Documentation build had issues${NC}"
fi

echo ""
echo "5. Checking for TODO comments..."
echo "-------------------------------"
TODO_COUNT=$(grep -r "TODO\|FIXME\|HACK\|XXX" src/ --exclude-dir=target 2>/dev/null | wc -l || echo "0")
if [ "$TODO_COUNT" -gt 0 ]; then
    echo -e "${YELLOW}⚠ Found $TODO_COUNT TODO/FIXME comments:${NC}"
    grep -r "TODO\|FIXME\|HACK\|XXX" src/ --exclude-dir=target || true
else
    echo -e "${GREEN}✓ No TODO comments found${NC}"
fi

echo ""
echo "6. Checking for unsafe code..."
echo "-------------------------------"
UNSAFE_COUNT=$(grep -r "unsafe" src/ --exclude-dir=target 2>/dev/null | wc -l || echo "0")
if [ "$UNSAFE_COUNT" -gt 0 ]; then
    echo -e "${YELLOW}⚠ Found $UNSAFE_COUNT uses of unsafe:${NC}"
    grep -r "unsafe" src/ --exclude-dir=target -n || true
else
    echo -e "${GREEN}✓ No unsafe code${NC}"
fi

echo ""
echo "7. Checking dependencies..."
echo "-------------------------------"
if cargo tree --depth=0 | grep -E "^[a-z]" > /dev/null; then
    echo -e "${GREEN}✓ Dependencies resolved${NC}"
    echo "Direct dependencies:"
    cargo tree --depth=1 | grep -E "^[├└]"
else
    echo -e "${RED}✗ Dependency issues detected${NC}"
fi

echo ""
echo "8. Checking file sizes..."
echo "-------------------------------"
LARGE_FILES=$(find src/ -name "*.rs" -type f -exec wc -l {} + | sort -rn | head -5)
echo "Largest source files (lines):"
echo "$LARGE_FILES"

echo ""
echo "9. Release build check..."
echo "-------------------------------"
if cargo build --release --quiet; then
    echo -e "${GREEN}✓ Release build successful${NC}"
    BINARY_SIZE=$(ls -lh target/release/z80asm | awk '{print $5}')
    echo "Binary size: $BINARY_SIZE"
else
    echo -e "${RED}✗ Release build failed${NC}"
    exit 1
fi

echo ""
echo "==================================="
echo -e "${GREEN}All checks passed!${NC}"
echo "==================================="

# Optional: Open documentation in browser
# cargo doc --open