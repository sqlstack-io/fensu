# Fensu website

The homepage at `/` and documentation at `/docs/` build as one Astro/Starlight site, using the
same layout foundations as SQLBuild and SQLClean with Fensu's original logo and orange accent.

- `public/index.html`, `public/styles.css` and `public/site.js`: semantic marketing homepage.
- `src/content/docs/docs/`: the 24 MDX pages imported from
  [fensu-docs](https://github.com/sqlstack-io/fensu-docs) at commit
  `d88c7e748a492b19b453bd3b8b6b4a8a7637544b`, with trailing whitespace normalized.
- `src/navigation.mjs`: the original sidebar order and groups, plus old-path redirects.
- `src/mintlify-content.mjs`: compile-time compatibility for imported components and links.
  Code samples and heading text are preserved. New documentation should use `/docs/` links.
- `src/components/` and `src/styles/`: sibling-family layout, typography and Fensu theme.

Pagefind search and `llms.txt`, `llms-small.txt` and `llms-full.txt` are generated at build time.
Search works on the built preview, not the development server. See [migration details](MIGRATION.md)
for URL mappings and the separate live-docs cutover requirements.

## Develop and check

Requires Node 22.23.4 or newer and Python 3.

```sh
npm ci
npm run dev
npx playwright install chromium
npm test
```

Tests build and serve a separate copy on port 4174. `CHROME_PATH` can select an existing Chromium
executable. Tests cover built page/anchor links and assets, local search, code tabs, keyboard
navigation, copying, cross-page theme persistence, responsive layout and accessibility in both themes.

## Build

```sh
npm run build
```

The build writes `dist/`, sets absolute canonical and sharing URLs, and preserves queries/fragments
on static legacy-path redirects. Serve `dist/` with a static server or run `npm run preview`.
There is no production deployment workflow in this directory.

Origin precedence is optional `--site https://example.com`, then `SITE_URL`, then Cloudflare's
`CF_PAGES_URL`, then the non-production local fallback `http://localhost:4321`. Values must be
HTTP(S) origins without paths, queries or credentials. Set `SITE_URL` to the approved production
origin in the production environment when the final domain is known; leave it unset in preview
environments so their canonical and sharing URLs follow their actual deployment URLs.

For Cloudflare Pages, set the root directory to `website`, output directory to `dist`,
build command to `npm run build`, and `NODE_VERSION` to `22` (the same major as CI, at least
22.23.4 as required by the locked Astro version). Pages installs the locked
dependencies automatically and provides `CF_PAGES_URL`. No secrets or other environment variables
are required for a Pages preview. Do not set a guessed custom production domain.

CI builds the unified site, checks internal pages/anchors and runs browser tests for website-only
changes. Standalone Markdown changes run local-link checks. Mixed, unknown and workflow changes
retain all product checks. The always-running `verify` aggregate requires every selected job to
succeed and accepts only intentional skips. Release Please excludes commits confined to `website/`
or standalone `docs/`; runtime and bundled skill changes remain releasable.

## Assets and copy

- `public/assets/fensu-original.png` preserves the supplied original logo byte for byte. The
  light/dark lockups and icons are cropped raster derivatives, not redrawn marks. The white ink
  becomes charcoal in the light version; orange is preserved.
- Inter and IBM Plex Mono are self-hosted under `public/fonts/` with their license notices.
- `public/assets/og-fensu.png` is the sharing image. It uses the original raster mark and the
  homepage headline, not a screenshot of an application.
- Homepage claims follow the released commands and public documentation. Call mapping is described
  as Python-specific; duplicated-code detection is advisory. The Python domain tree is an
  architecture illustration, not fabricated CLI output.
