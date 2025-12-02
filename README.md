# fzj

A command-line tool for fuzzy selection over JSON inputs with flexible field filtering and output control.

## Overview

`fzj` (fuzzy JSON) provides an interactive fuzzy-search interface for JSON data piped from stdin.
It allows you to quickly filter and select items from JSON arrays, with options to control which fields are displayed during selection and which fields appear in the final output.

## Installation

TBD

## Usage

### Basic Usage

`fzj` reads JSON arrays from stdin, presents an interactive fuzzy-search interface, and outputs the selected JSON object to stdout.

```bash
RAW_JSON='[{"name": "foo", "count": 4}, {"name": "far", "count": 8}, {"name": "baz", "count": 1}]'

echo $RAW_JSON | fzj
# User types 'fo' and hits enter
# Output: {"name": "foo", "count": 4}
```

### Field Filtering (`--fields`)

Control which fields are displayed during the selection interface. The full object is still output.

```bash
echo $RAW_JSON | fzj --fields name
# Shows only the 'name' field during selection
# Still outputs the complete selected object
```

Multiple fields can be specified in any order:

```bash
echo $RAW_JSON | fzj --fields count,name
# Shows 'count' and 'name' fields during selection
```

**Note**: If `--fields` is not specified, all fields from the JSON objects are displayed.

### Nested Array Extraction (`--dig`)

When your input is not a top-level array but contains nested arrays, use `--dig` to extract the array first.

```bash
NESTED='{"results": [{"id": 1, "title": "First"}, {"id": 2, "title": "Second"}]}'

echo $NESTED | fzj --dig results
# Extracts the 'results' array and allows selection from its items
```

Works with deeper nesting using dot notation:

```bash
DEEP='{"data": {"users": [{"name": "Alice"}, {"name": "Bob"}]}}'

echo $DEEP | fzj --dig data.users
```

### Output Field Selection (`--out`)

Limit the output to only specific fields from the selected object.

```bash
echo $RAW_JSON | fzj --out name
# Output: {"name": "foo"}
```

Multiple output fields:

```bash
echo $RAW_JSON | fzj --out name,count
# Output: {"name": "foo", "count": 4}
```

### Combined Options

All options can be used together for maximum flexibility:

```bash
# Display only 'name' during selection, output only 'count'
echo $RAW_JSON | fzj --fields name --out count

# Extract nested array, filter displayed fields, and limit output
echo $NESTED | fzj --dig results --fields title --out id
```
