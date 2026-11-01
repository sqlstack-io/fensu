import { defineConfig } from 'astro/config';
import { unified } from '@astrojs/markdown-remark';
import starlight from '@astrojs/starlight';
import starlightLlmsTxt from 'starlight-llms-txt';
import { sidebar, redirects } from './src/navigation.mjs';
import { fensuDark, fensuLight } from './src/code-themes.mjs';
import mintlifyContent from './src/mintlify-content.mjs';

const site = process.env.SITE_URL || process.env.CF_PAGES_URL || 'http://localhost:4321';

export default defineConfig({
  site,
  redirects,
  markdown: { processor: unified({ remarkPlugins: [mintlifyContent] }) },
  integrations: [starlight({
    title: 'Fensu',
    description: 'Executable architecture rules for Python, Rust, TypeScript and Svelte repositories.',
    logo: { light: './src/assets/logo-light.png', dark: './src/assets/logo-dark.png', replacesTitle: true },
    favicon: '/assets/icon-light.png',
    head: [
      { tag: 'link', attrs: { rel: 'icon', type: 'image/png', href: '/assets/icon-dark.png', media: '(prefers-color-scheme: dark)' } },
      { tag: 'link', attrs: { rel: 'apple-touch-icon', href: '/assets/apple-touch-icon.png' } },
      { tag: 'meta', attrs: { property: 'og:image', content: `${site}/assets/og-fensu.png` } },
      { tag: 'meta', attrs: { property: 'og:image:width', content: '1200' } },
      { tag: 'meta', attrs: { property: 'og:image:height', content: '630' } },
      { tag: 'meta', attrs: { name: 'twitter:card', content: 'summary_large_image' } },
      { tag: 'meta', attrs: { name: 'twitter:image', content: `${site}/assets/og-fensu.png` } },
    ],
    social: [{ icon: 'github', label: 'Fensu on GitHub', href: 'https://github.com/sqlstack-io/fensu' }],
    components: {
      PageFrame: './src/components/PageFrame.astro',
      TwoColumnContent: './src/components/TwoColumnContent.astro',
      Header: './src/components/Header.astro',
      PageTitle: './src/components/PageTitle.astro',
      SiteTitle: './src/components/SiteTitle.astro',
      Pagination: './src/components/Pagination.astro',
      ThemeSelect: './src/components/ThemeSelect.astro',
    },
    customCss: ['./src/styles/fonts.css', './src/styles/fensu.css'],
    expressiveCode: {
      themes: [fensuDark, fensuLight],
      defaultProps: { overridesByLang: { 'bash,sh,shell,zsh,console,powershell': { frame: 'none' } } },
      styleOverrides: {
        borderRadius: '2px', borderWidth: '1px', borderColor: 'var(--sqb-line)',
        codeFontFamily: 'var(--sl-font-mono)', codeFontSize: '13px', codeLineHeight: '1.65',
        uiFontFamily: 'var(--sl-font)',
        frames: { shadowColor: 'transparent', frameBoxShadowCssValue: 'none' },
      },
    },
    sidebar,
    plugins: [starlightLlmsTxt({
      projectName: 'Fensu',
      description: 'Fensu enforces code boundaries and repository structure with executable rules.',
    })],
  })],
});
