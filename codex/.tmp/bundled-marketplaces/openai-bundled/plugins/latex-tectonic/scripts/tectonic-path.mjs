import { existsSync } from "node:fs";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

export function getTectonicExecutablePath(
  pluginRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), ".."),
) {
  const executableName =
    process.platform === "win32" ? "tectonic.exe" : "tectonic";
  const executablePath = path.join(pluginRoot, "bin", executableName);
  if (!existsSync(executablePath)) {
    throw new Error(
      `Bundled Tectonic executable not found at ${executablePath}.`,
    );
  }
  return executablePath;
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  console.log(getTectonicExecutablePath());
}
