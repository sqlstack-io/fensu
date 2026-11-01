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

Astro's `site` is set directly to `https://fensu.io`. Canonical, sharing and sitemap URLs always
use this production origin, including local and branch-preview builds. Navigation, assets and
search still work on the host serving the preview. No site-origin environment variable or CLI
argument is required; existing `SITE_URL` and `CF_PAGES_URL` values are ignored.

For Cloudflare Pages, set the root directory to `website`, output directory to `dist`,
build command to `npm run build`, and `NODE_VERSION` to `22` (the same major as CI, at least
22.23.4 as required by the locked Astro version). Pages installs the locked
dependencies automatically. No secrets or site-origin dashboard variables are required.
An existing dashboard `SITE_URL` can be removed at any time.

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
- `public/assets/og-fensu-social.png` is the sharing image. It uses the original raster mark and
  homepage headline on a near-black background, not an application screenshot. Important content
  stays within the centered 630-pixel square of the 1200 × 630 image for small square previews.
  The previous image remains available for cached links.
- Homepage claims follow the released commands and public documentation. Call mapping is described
  as Python-specific; duplicated-code detection is advisory. The Python domain tree is an
  architecture illustration, not fabricated CLI output.
