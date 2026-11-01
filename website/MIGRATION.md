# Documentation migration

The source is [sqlstack-io/fensu-docs](https://github.com/sqlstack-io/fensu-docs), commit
`d88c7e748a492b19b453bd3b8b6b4a8a7637544b`. Its 24 MDX pages, sidebar group/order and two call-map
images are preserved here. One trailing space was normalized in `adoption.mdx`; all other MDX
files are byte-identical. The original repository and Mintlify deployment are unchanged.

## Route mapping

Paths below are relative to the old docs origin and the new unified site origin respectively.
The production origin is a hosting decision, not set by this migration.

| Old path | Unified path |
| --- | --- |
| `/`, `/index` | `/docs/` |
| `/quickstart` | `/docs/quickstart/` |
| `/adoption` | `/docs/adoption/` |
| `/philosophy` | `/docs/philosophy/` |
| `/concepts/architecture-model` | `/docs/concepts/architecture-model/` |
| `/concepts/configuration` | `/docs/concepts/configuration/` |
| `/concepts/caching` | `/docs/concepts/caching/` |
| `/concepts/rule-families` | `/docs/concepts/rule-families/` |
| `/concepts/duplicated-code` | `/docs/concepts/duplicated-code/` |
| `/concepts/rule-packs` | `/docs/concepts/rule-packs/` |
| `/concepts/rule-packs/dagster` | `/docs/concepts/rule-packs/dagster/` |
| `/concepts/custom-rules` | `/docs/concepts/custom-rules/` |
| `/concepts/repository-rules` | `/docs/concepts/repository-rules/` |
| `/concepts/analyzers/rust` | `/docs/concepts/analyzers/rust/` |
| `/concepts/analyzers/typescript-sveltekit` | `/docs/concepts/analyzers/typescript-sveltekit/` |
| `/cli`, `/cli/index` | `/docs/cli/` |
| `/cli/check` | `/docs/cli/check/` |
| `/cli/dupes` | `/docs/cli/dupes/` |
| `/cli/init` | `/docs/cli/init/` |
| `/cli/map` | `/docs/cli/map/` |
| `/cli/rule` | `/docs/cli/rule/` |
| `/cli/skills` | `/docs/cli/skills/` |
| `/cli/target` | `/docs/cli/target/` |
| `/reference/rule-context` | `/docs/reference/rule-context/` |

The unified build includes static redirects for these non-root old paths and `/docs/index` and
`/docs/cli/index` aliases. JavaScript preserves the query and fragment; the ordinary link and meta
refresh remain as a no-JavaScript fallback. The root `/` is the marketing homepage, not a redirect.
Heading IDs are generated from the unchanged headings; internal cross-page fragments are checked
against every built page. Code groups become keyboard-accessible Starlight tabs, cards and callouts
retain their content, and theme-specific call-map images remain available at their original paths.

## Before a live cutover

This branch does not change domains, publish a website or retire Mintlify. A later hosting change
must redirect requests on `docs.fensu.dev` to the unified origin using the mapping above, including
the old root to `/docs/`. Preserve query strings and fragments and use permanent HTTP redirects
where the host supports them. Keep the two call-map image URLs accessible for existing references.
Do not redirect `/docs/*` to `/docs/docs/*`.

After deploying a built site, check the old-host redirects, canonical URLs, Pagefind search and
mobile navigation on the final origin. Until cutover, the live Mintlify docs remain authoritative;
coordinate content changes there with this imported snapshot to avoid losing edits.
