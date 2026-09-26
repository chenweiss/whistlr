#!/usr/bin/env node
const { version } = require("../package.json");

if (process.argv.includes("--version")) {
  console.log(`whistlr ${version}`);
} else {
  console.log("usage: whistlr [--version]");
}
