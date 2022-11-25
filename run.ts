const args = Deno.args;
const day = args[0];
const part = args[1] || "both";
const year = args[2] || "2022";

const dir = `./${year}/${day}`;

// check if the directory exists
const dirExists = await Deno.stat(dir)
    .then(() => true)
    .catch(() => false);

if (!dirExists) {
    console.log(`Directory ${dir} does not exist`);
    Deno.exit(1);
}

// check if a.ts exists and run it
const aExists = await Deno.stat(`${dir}/a.ts`)
    .then(() => true)
    .catch(() => false);

if (aExists && (part === "a" || part === "both")) {
    console.log(`Running ${dir}/a.ts`);
    Deno.chdir(dir);
    await Deno.run({
        cmd: ["deno", "run", "-A", `./a.ts`],
    }).status();
    Deno.chdir("../../");
}

// check if b.ts exists and run it
const bExists = await Deno.stat(`${dir}/b.ts`)
    .then(() => true)
    .catch(() => false);

if (bExists && (part === "b" || part === "both")) {
    console.log(`Running ${dir}/b.ts`);
    Deno.chdir(dir);
    await Deno.run({
        cmd: ["deno", "run", "-A", `./b.ts`],
    }).status();
    Deno.chdir("../../");
}
