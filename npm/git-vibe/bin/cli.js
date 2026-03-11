#!/usr/bin/env node

const { execFileSync } = require("child_process");
const { join } = require("path");

const PLATFORMS = {
  "darwin-arm64": "@calintzy/cli-darwin-arm64",
  "darwin-x64": "@calintzy/cli-darwin-x64",
  "linux-x64": "@calintzy/cli-linux-x64",
  "linux-arm64": "@calintzy/cli-linux-arm64",
  "win32-x64": "@calintzy/cli-win32-x64",
  "win32-arm64": "@calintzy/cli-win32-arm64",
};

function getBinaryPath() {
  const platform = process.platform;
  const arch = process.arch === "arm64" ? "arm64" : "x64";
  const key = `${platform}-${arch}`;
  const pkg = PLATFORMS[key];

  if (!pkg) {
    throw new Error(
      `Unsupported platform: ${platform}-${arch}\n` +
        `git-vibe supports: ${Object.keys(PLATFORMS).join(", ")}\n` +
        `You can install from source: cargo install git-vibe`
    );
  }

  try {
    const binDir = require.resolve(`${pkg}/package.json`);
    const ext = platform === "win32" ? ".exe" : "";
    return join(binDir, "..", `git-vibe${ext}`);
  } catch {
    throw new Error(
      `Platform package ${pkg} is not installed.\n` +
        `Try reinstalling: npm install -g @calintzy/git-vibe\n` +
        `Or install from source: cargo install git-vibe`
    );
  }
}

try {
  const binPath = getBinaryPath();
  const result = execFileSync(binPath, process.argv.slice(2), {
    stdio: "inherit",
    encoding: "utf-8",
  });
} catch (err) {
  if (err.status !== undefined) {
    process.exit(err.status);
  }
  console.error(err.message);
  process.exit(1);
}
