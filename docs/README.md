# fmerge

`fmerge` is a tool that allows merging files **recursively** and with custom regex patterns. The include file statements are always relative to the file that includes them.

## Regex matches

### Capture groups
- 1 (required): The relative file path for the file to be included.
- 2 (optional): The indentation of the included file in the current one (note that this excludes the first line).

## Example

- `fmerge merge -p "\{\{\s*([\w./]+)\s*\+?(\d+)?\s*\}\}" ./root.part`\
  Pattern that is matched: `{{ relative_path +indentation }}` => `{{ leaf.part +2 }}`
