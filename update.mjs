#!/usr/bin/env zx

import "zx/globals";

$.env.CARGO_TERM_COLOR = "always";

// download servers.json

const servers = await fetch(
    "https://servermappings.lunarclientcdn.com/servers.json",
).then((s) => s.text());
await fs.writeFile("servers.json", servers);

// update version metadata

const cargoToml = await fs.readFile("Cargo.toml", "utf8");
const now = new Date().toISOString().split("T")[0];
const newCargoToml = cargoToml.replace(
    /version = "(.*)"/,
    `version = "2.0.0+${now}"`,
);
await fs.writeFile("Cargo.toml", newCargoToml);

// test

await $`cargo build`;
await $`cargo test`;

// commit

const res = await question("Commit to git? [y/n] ");
if (res === "y") {
    await $`git add .`;
    await $`git commit -m "chore: update submodules"`;
}
