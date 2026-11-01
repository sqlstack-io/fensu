import assert from 'node:assert/strict';
import { test } from 'node:test';
import { selectChecks, validateResults } from './ci-checks.mjs';

test('only marketing and standalone docs changes skip product checks', () => {
  for (const [paths, expected] of [
    [['website/public/index.html'], { full: false, content: true, website: true }],
    [['website/src/content/docs/docs/quickstart.mdx'], { full: false, content: true, website: true }],
    [['README.md', 'docs/rust.md'], { full: false, content: true, website: false }],
    [['website/public/styles.css', 'docs/dupes.md'], { full: false, content: true, website: true }],
    [['website/public/index.html', 'crates/fensu-cli/src/main.rs'], { full: true, content: true, website: true }],
    [['.github/workflows/verify.yml'], { full: true, content: true, website: true }],
    [['crates/fensu-cli/skill/SKILL.md'], { full: true, content: true, website: true }],
    [['new-unknown-file'], { full: true, content: true, website: true }],
    [[], { full: true, content: true, website: true }],
  ]) assert.deepEqual(selectChecks(paths), expected, paths.join(', '));
});

test('aggregate accepts intentional skips, but never failed, cancelled or missing selected checks', () => {
  const docs = { changes: { result: 'success', outputs: { full: 'false', website: 'false' } }, content: { result: 'success' }, product: { result: 'skipped' } };
  assert.doesNotThrow(() => validateResults(docs));
  assert.throws(() => validateResults({ ...docs, changes: { result: 'failure' } }));
  assert.throws(() => validateResults({ ...docs, changes: { result: 'success', outputs: {} } }));
  assert.throws(() => validateResults({ ...docs, content: { result: 'skipped' } }));
  assert.throws(() => validateResults({ ...docs, product: { result: 'cancelled' } }));
  assert.throws(() => validateResults({ ...docs, website: { result: 'failure' } }));
  assert.throws(() => validateResults({ ...docs, changes: { result: 'success', outputs: { full: 'false', website: 'true' } } }));
  assert.throws(() => validateResults({ ...docs, changes: { result: 'success', outputs: { full: 'true', website: 'true' } } }));
  const full = Object.fromEntries(['content', 'website', 'product', 'rust', 'shared-crates', 'performance', 'windows', 'native-package-smoke', 'web-parser-analyzer-smoke', 'source-build'].map((job) => [job, { result: 'success' }]));
  full.changes = { result: 'success', outputs: { full: 'true', website: 'true' } };
  assert.doesNotThrow(() => validateResults(full));
  for (const result of ['skipped', 'failure', 'cancelled']) assert.throws(() => validateResults({ ...full, rust: { result } }));
});
