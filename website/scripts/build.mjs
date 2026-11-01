import { readFile, writeFile } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
import { redirects } from '../src/navigation.mjs';

const root = new URL('../', import.meta.url);
const output = new URL('dist/', root);
execFileSync('npm', ['exec', '--', 'astro', 'build'], { cwd: root, stdio: 'inherit' });
// Astro's meta refresh drops queries and fragments. Run this first, keeping its
// ordinary link/meta-refresh fallback for browsers without JavaScript.
for (const [from, to] of Object.entries(redirects)) {
  const file = new URL(`${from.replace(/^\//, '').replace(/\/$/, '')}/index.html`, output);
  const redirect = await readFile(file, 'utf8');
  await writeFile(file, redirect.replace('<meta http-equiv=', `<script>location.replace(${JSON.stringify(to)} + location.search + location.hash)</script><meta http-equiv=`));
}
