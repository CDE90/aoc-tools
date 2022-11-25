// run command deno new <day> <year>
// and copy the template directory for the day
// compile using deno compile --allow-read --allow-write --allow-env --allow-net new.ts
// and then run the executable

import { config } from "https://deno.land/std@0.166.0/dotenv/mod.ts";

const args = Deno.args;
const day = args[0];
const year = args[1] || "2022";

const configData = await config();
const cookie = configData["AOC_SESSION"];

const url = `https://adventofcode.com/${year}/day/${day}/input`;
const headers = new Headers();
headers.set("Cookie", `session=${cookie}`);

const response = await fetch(url, { headers });
const text = await response.text();

const dir = `./${year}/${day}`;
const file = `${dir}/input.txt`;

// if the directory doesn't exist, create it

const dirExists = await Deno.stat(dir)
    .then(() => true)
    .catch(() => false);

if (!dirExists) {
    await Deno.mkdir(dir, { recursive: true })
        .then(() => Deno.writeTextFile(file, text))
        .then(() => console.log(`Created ${file}`))
        .then(() => Deno.copyFile("./template/a.ts", `${dir}/a.ts`))
        .then(() => console.log(`Created ${dir}/a.ts`))
        .then(() => Deno.copyFile("./template/b.ts", `${dir}/b.ts`))
        .then(() => console.log(`Created ${dir}/b.ts`))
        .catch((err) => console.error(err));
} else {
    console.log(`Directory ${dir} already exists`);
}
