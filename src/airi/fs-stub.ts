export function existsSync() { return false; }
export function mkdirSync() {}
export function writeFileSync() {}
export function readFileSync() { return ''; }
export function appendFileSync() {}
export function statSync() { return null; }
export function readdirSync() { return []; }
export function unlinkSync() {}
export function rmdirSync() {}
export function realpathSync() { return ''; }
export async function readFile() { return ''; }
export async function writeFile() {}
export async function access() { return undefined; }
export async function mkdir() {}
export async function readdir() { return []; }
export async function stat() { return null; }
export async function lstat() { return null; }
export async function unlink() {}
export async function rmdir() {}
export async function readlink() { return ''; }
export async function symlink() {}
export async function chmod() {}
export async function chown() {}
export async function appendFile() {}
export async function copyFile() {}
export async function exists() { return false; }
export async function realpath() { return ''; }
export default { readFile, writeFile };
