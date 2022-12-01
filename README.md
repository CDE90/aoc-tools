# My Advent of Code Things

This repo contains my [AoC](https://adventofcode.com/) tools to automatically download the input, create boilerplate code for rust, typescript or python, and run the code. To use it, you need to have [rust](https://www.rust-lang.org/tools/install) installed.

## Scripts

To automatically download the input, you will need to create a `.env` file in the root of the repo with the following content:

```bash
AOC_SESSION=""
```

You can get the session cookie from your browser. In Chrome, you can do this by visiting the [Advent of Code Website](https://adventofcode.com/) opening the developer tools, going to the application tab, and then copying the value of the `session` cookie.

### Installing

You must have [rust](https://www.rust-lang.org/tools/install) installed.

```bash
cargo install --path .
```

### New

Use this to create a new solution for a given day. This also downloads your personal input if you have `AOC_SESSION` set (in `.env` file). For example, to create a new typescript solution for day 1, run `aoc n -d 1 -l ts`. The possible arguments are listed below.

**Usage**:

```bash
aoc new [OPTIONS] --day <DAY> --lang <LANG>
```

### Run

Use this to run a solution for a given day. For example, to run the typescript solution for day 1, run `aoc r -d 1 -l ts`. The possible arguments are listed below.

**Usage**:

```bash
aoc run [OPTIONS] --day <DAY> --lang <LANG>
```

| Argument    | Type      | Description                                                  |
|------------ |---------- |------------------------------------------------------------- |
| -d, --day   | `<DAY>`   | Day to create/run                                            |
| -y, --year  | `<YEAR>`  | Year to create/run (Optional: defaults to most recent AOC) [can't use with run]    |
| -l, --lang  | `<LANG>`  | Language to use [default: ts] [possible values: rs, ts, py]  |
| -h, --help  |           | Print help information                                       |

## License

[MIT](LICENSE)
