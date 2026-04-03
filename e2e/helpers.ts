import os from "node:os";
import path from "node:path";
import fs from "node:fs";

export const isWindows = os.platform() === "win32";
export const isMacOS = os.platform() === "darwin";

export const pathSep = isWindows ? ";" : ":";

export const paveBin = isWindows
  ? path.resolve(process.cwd(), "..", "target", "debug", "pave.exe")
  : path.resolve(process.cwd(), "..", "target", "debug", "pave");

export function testEnv(
  overrides: Record<string, string>
): Record<string, string | undefined> {
  const env: Record<string, string | undefined> = { ...process.env };
  if (isWindows) {
    for (const key of Object.keys(overrides)) {
      const keyLower = key.toLowerCase();
      for (const envKey of Object.keys(env)) {
        if (envKey.toLowerCase() === keyLower && envKey !== key) {
          delete env[envKey];
        }
      }
    }
  }
  return { ...env, ...overrides };
}

export function createFakeExecutable(name: string): {
  dir: string;
  execPath: string;
} {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "pave-test-exec-"));
  const execName = isWindows ? `${name}.cmd` : name;
  const execPath = path.join(dir, execName);
  fs.writeFileSync(execPath, isWindows ? "@echo off\n" : "#!/bin/sh\n");
  if (!isWindows) {
    fs.chmodSync(execPath, 0o755);
  }
  return { dir, execPath };
}
