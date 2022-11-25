# My Advent of Code Things

This repo contains my [AoC](https://adventofcode.com/) solutions, and also my scripts to automatically download the input for each day & initialise basic ts files for parts 1 & 2.

To use the scripts, you'll need to have [Deno](https://deno.land/) installed.

## Scripts

### `new.ts`

This script will create a new folder for the current day, and create a `a.ts` and `b.ts` file in it, with the very basic boilerplate code for each part. It will also download the input for the day, and save it to a file called `input.txt`.

To run this script, run `deno run --allow-net --allow-read --allow-write new.ts <day> <year>` from the root of the repo. Or, compile it to a binary with `deno compile --allow-net --allow-read --allow-write new.ts` and run the executable from the root of the repo.

### `run.ts`

This script will run the solution for the current day. It will run both parts by default (or specify a or b after the command), and print the results to the console.

To run this script, run `deno run --allow-read --allow-run run.ts <day> <part> <year>` from the root of the repo. Or, compile it to a binary with `deno compile --allow-read --allow-run run.ts` and run the executable from the root of the repo.

## License

[MIT](LICENSE)
