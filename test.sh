#!/bin/bash

echo "Test 1: Basic usage with selection (selecting first item with enter)"
echo '[{"name": "foo", "count": 4}, {"name": "far", "count": 8}, {"name": "baz", "count": 1}]' | cargo run

echo ""
echo "Test 2: Using --fields to display only specific fields"
echo '[{"name": "foo", "count": 4, "extra": "data"}, {"name": "far", "count": 8, "extra": "more"}]' | cargo run -- --fields name,count

echo ""
echo "Test 3: Using --out to filter output fields"
echo '[{"name": "foo", "count": 4, "extra": "data"}, {"name": "far", "count": 8, "extra": "more"}]' | cargo run -- --out name

echo ""
echo "Test 4: Using --dig to extract nested array"
cat test_nested.json | cargo run -- --dig data.items

echo ""
echo "Test 5: Combining --dig and --fields"
cat test_nested.json | cargo run -- --dig data.items --fields name,price
