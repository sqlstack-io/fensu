import { readFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';

const productJobs = ['product', 'rust', 'shared-crates', 'performance', 'windows', 'native-package-smoke', 'web-parser-analyzer-smoke', 'source-build'];

export function selectChecks(paths) {
  const contentOnly = (path) => path.startsWith('docs/') || ['README.md', 'CHANGELOG.md', 'LICENSE', '.github/fensu-logo-wide.png'].includes(path);
  const full = paths.length === 0 || paths.some((path) => !path.startsWith('website/') && !contentOnly(path));
  return {
    full,
    content: true,
    website: full || paths.some((path) => path.startsWith('website/')),
  };
}

export function validateResults(jobs) {
  if (jobs.changes?.result !== 'success') throw new Error('Change selection did not succeed');
  for (const key of ['full', 'website']) {
    if (!['true', 'false'].includes(jobs.changes.outputs?.[key])) throw new Error(`Missing or invalid ${key} selection`);
  }
  for (const [name, job] of Object.entries(jobs)) {
    if (!['success', 'skipped'].includes(job.result)) throw new Error(`${name}: ${job.result}`);
  }
  const required = ['content'];
  if (jobs.changes.outputs.full === 'true') required.push(...productJobs);
  if (jobs.changes.outputs.website === 'true') required.push('website');
  for (const name of required) {
    if (jobs[name]?.result !== 'success') throw new Error(`Selected check ${name} did not succeed`);
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  if (process.argv[2] === 'verify') {
    validateResults(JSON.parse(process.env.CHECK_RESULTS));
    console.log('All selected checks passed; unselected checks skipped');
  } else {
    const paths = process.argv.includes('--all') ? [] : readFileSync(0, 'utf8').split('\0').filter(Boolean);
    for (const [key, value] of Object.entries(selectChecks(paths))) console.log(`${key}=${value}`);
  }
}
