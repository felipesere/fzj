# fjz

A tool to fuzzy select over JSON inputs.
It also outputs JSON.


## Usage

`fzj` reads JSON from stdin, presents it to the user to fuzzy select from and then prints the select JSON object.

```bash
RAW_JSON="[{name: "foo", count: 4}, {name: "far", count: 8}, {name: "baz", count: 1}]"

echo $RAW_JSON  | fzj
# User types in 'fo' and hits enter
# fzj will write {name: "foo", count: 4} to stdout
```

The user can use `--fields a,c,b` to only present the values of `a`,`b`, and `c` to the user.
`fzj` will still output the selected object unchanged.
If `--fields` is not passed then all fields of the objects in the array are presented.

If the object passed in is not an array, but contains a nested array somewhere, the user can use `--dig $path` to extract the array first and then present the fields.

If needed, the user can also pass `--out x,y,z` to only ouput those fields of the final object.
