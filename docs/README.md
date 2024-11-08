# fmerge

`fmerge` is a tool that allows merging files **recursively** and with custom placeholder patterns. The include file statements are always relative to the file that includes them.

## Example

### Test data

root.json
```json:root.json
{
    "data": [
        {{ ./item1.json }},
        {{ ./item2.json }}
    ]
}
```
item1.json
```json
{
    "name": "Item 1",
    "data": {{ ./item_data.json }}
}
```
item2.json
```json
{
    "name": "Item 2",
    "data": {{ ./item_data.json }}
}
```
item_data.json
```json
{
    "foo": "bar"
}
```

### Execution

Merging these files together can be done by executing the following code:

```bash
fmerge merge -f=./root.json -r="{{ (.*) (\+?[0-9]*) }}"
```

The resulting file will be printed to `STDOUT` and will look like this:
```json
{
    "data": [
        {
    "name": "Item 1",
    "data": {
    "foo": "bar"
}
},
        {
    "name": "Item 2",
    "data": {
    "foo": "bar"
}
}
    ]
}
```

The text replacement is done without respect to the formatting. The structure above is valid JSON, just formatted incorrectly. `fmerge` does not modify the content it merges in any way, shape or form.\
The correctly formatted  JSON looks as follows:

```json
{
  "data": [
    {
      "name": "Item 1",
      "data": {
        "foo": "bar"
      }
    },
    {
      "name": "Item 2",
      "data": {
        "foo": "bar"
      }
    }
  ]
}

```
