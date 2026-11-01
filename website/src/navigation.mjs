export const sidebar = [
  { label: 'Getting Started', items: [
    { label: 'Fensu (フェンス)', slug: 'docs' },
    { label: 'Quickstart', slug: 'docs/quickstart' },
    { label: 'Adopting Fensu', slug: 'docs/adoption' },
    { label: 'Philosophy', slug: 'docs/philosophy' },
  ] },
  { label: 'Concepts', items: [
    { label: 'Architecture Model', slug: 'docs/concepts/architecture-model' },
    { label: 'Configuration', slug: 'docs/concepts/configuration' },
    { label: 'Caching', slug: 'docs/concepts/caching' },
    { label: 'Rule Families', slug: 'docs/concepts/rule-families' },
    { label: 'Duplicated Code', slug: 'docs/concepts/duplicated-code' },
    { label: 'Native Rule Packs', items: [
      { label: 'Overview', slug: 'docs/concepts/rule-packs' },
      { label: 'Dagster', slug: 'docs/concepts/rule-packs/dagster' },
    ] },
    { label: 'Custom Rules', slug: 'docs/concepts/custom-rules' },
    { label: 'Repository Rules', slug: 'docs/concepts/repository-rules' },
    { label: 'Additional Analyzers', items: [
      { label: 'Rust', slug: 'docs/concepts/analyzers/rust' },
      { label: 'TypeScript and SvelteKit', slug: 'docs/concepts/analyzers/typescript-sveltekit' },
    ] },
  ] },
  { label: 'CLI Reference', items: [
    { label: 'Overview', slug: 'docs/cli' },
    ...['check', 'dupes', 'init', 'map', 'rule', 'skills', 'target'].map((command) => ({ label: `fensu ${command}`, slug: `docs/cli/${command}` })),
  ] },
  { label: 'API Reference', items: [{ label: 'RuleContext', slug: 'docs/reference/rule-context' }] },
];

const slugs = (items) => items.flatMap((item) => item.items ? slugs(item.items) : [item.slug]);
export const docsRoutes = slugs(sidebar).map((slug) => `/${slug}/`);
export const redirects = Object.fromEntries(docsRoutes.filter((route) => route !== '/docs/').map((route) => [route.replace('/docs', ''), route]));
Object.assign(redirects, { '/index': '/docs/', '/cli/index': '/docs/cli/', '/docs/index': '/docs/', '/docs/cli/index': '/docs/cli/' });
