# pixterm
A CLI to show images in a terminal

## Usage
`pixterm [FLAGS] [OPTIONS] <file>`

### Arguments
| name   | description                   |
| ------ | ----------------------------- |
| `file` | Path to image file to display |

### Flags
| short | long        | description                                     |
| ----- | ----------- | ----------------------------------------------- |
| `-h`  | `--help`    | Prints help information                         |
| `-r`  | `--raw`     | Print escape sequences literal                  |
| `-s`  | `--silent`  | Do not print to stdout. Useful with `--outfile` |
| `-V`  | `--version` | Prints version information                      |

### Options
| short | long          | description                                                                                                            | default |
| ----- | ------------- | ---------------------------------------------------------------------------------------------------------------------- | ------- |
| `-W`  | `--width`     | Maximum width in pixels of the resized image. Also see `--height`                                                      | 32      |
| `-H`  | `--height`    | Maximum height in pixels of the resized image. Also see `--width`                                                      | 32      |
| `-o`  | `--outfile`   | File to write the resulting string into. See `--raw` to get literal escape sequences and `--silent` to suppress stdout | None    |
| `-t`  | `--threshold` | Minimum alpha value of a pixel for it to be shown                                                                      | 50      |
