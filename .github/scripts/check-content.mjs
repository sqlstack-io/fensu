import { readFile, stat } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
import { dirname, resolve } from 'node:path';

const files = execFileSync('git', ['ls-files', '-z', 'README.md', 'docs/*.md'], { encoding: 'utf8' }).split('\0').filter(Boolean);
let checked = 0;
for (const file of files) {
  const markdown = (await readFile(file, 'utf8')).replace(/```[^]*?```/g, '');
  for (const match of markdown.matchAll(/\]\(([^\s)]+)\)/g)) {
    const href = match[1];
    if (/^(?:https?:|mailto:|#)/.test(href)) continue;
    const target = resolve(dirname(file), decodeURIComponent(href.split('#')[0]));
    if (!(await stat(target)).isFile()) throw new Error(`Broken link in ${file}: ${href}`);
    checked++;
  }
}
console.log(`Checked ${files.length} documentation files and ${checked} local links`);
