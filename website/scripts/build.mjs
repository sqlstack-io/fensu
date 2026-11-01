import { readFile, writeFile } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { redirects } from '../src/navigation.mjs';

const argument = process.argv.indexOf('--site');
if (argument >= 0 && !process.argv[argument + 1]) throw new Error('--site requires an HTTP(S) origin');
const site = new URL((argument >= 0 ? process.argv[argument + 1] : undefined)
  || process.env.SITE_URL || process.env.CF_PAGES_URL || 'http://localhost:4321');
if (!['http:', 'https:'].includes(site.protocol) || site.pathname !== '/' || site.search || site.hash || site.username || site.password) {
  throw new Error('Site URL must be an HTTP(S) origin, without a path, query or credentials');
}
const root = new URL('../', import.meta.url);
const output = new URL('dist/', root);
execFileSync('npm', ['exec', '--', 'astro', 'build', '--site', site.origin], {
  cwd: root, stdio: 'inherit', env: { ...process.env, SITE_URL: site.origin },
});
const html = await readFile(new URL('index.html', output), 'utf8');
await writeFile(new URL('index.html', output), html.replaceAll('__SITE_URL__', site.origin));
// Astro's meta refresh drops queries and fragments. Run this first, keeping its
// ordinary link/meta-refresh fallback for browsers without JavaScript.
for (const [from, to] of Object.entries(redirects)) {
  const file = new URL(`${from.replace(/^\//, '').replace(/\/$/, '')}/index.html`, output);
  const redirect = await readFile(file, 'utf8');
  await writeFile(file, redirect.replace('<meta http-equiv=', `<script>location.replace(${JSON.stringify(to)} + location.search + location.hash)</script><meta http-equiv=`));
}
console.log(`Built ${fileURLToPath(output)} for ${site.origin}`);
