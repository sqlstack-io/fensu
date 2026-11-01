import { test, expect, devices } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { docsRoutes, redirects } from '../src/navigation.mjs';

test('every documentation route, sidebar entry and legacy redirect resolves', async ({ page, request, baseURL }) => {
  for (const route of docsRoutes) {
    const response = await request.get(route);
    expect(response.ok(), route).toBe(true);
    const html = await response.text();
    expect(html).toContain('data-pagefind-body');
    expect(html).not.toContain('docs.fensu.dev/');
  }
  await page.goto('/docs/');
  const nav = page.getByRole('navigation', { name: 'Main', exact: true });
  const links = await nav.locator('a[href^="/docs"]').evaluateAll((elements) => elements.map((element) => element.getAttribute('href')));
  expect(new Set(links)).toEqual(new Set(docsRoutes));
  for (const [from, to] of Object.entries(redirects)) {
    await page.goto(`${from}?source=bookmark#_top`);
    await expect(page).toHaveURL(`${baseURL}${to}?source=bookmark#_top`);
    await expect(page.locator('h1')).toBeVisible();
  }
  const llms = await (await request.get('/llms-full.txt')).text();
  expect(llms).toContain('RuleContext');
  expect(llms).toContain('fensu skills --check');
});

test('local search finds an API concept and keyboard navigation opens it', async ({ page }) => {
  // A busy browser may never offer idle time after the search dialog opens.
  await page.addInitScript(() => {
    window.requestIdleCallback = (callback, options) => options?.timeout
      ? setTimeout(() => callback({ didTimeout: true, timeRemaining: () => 0 }), options.timeout)
      : 0;
  });
  await page.goto('/docs/quickstart/');
  await page.keyboard.press('Control+k');
  const dialog = page.getByRole('dialog');
  await expect(dialog).toBeVisible();
  await dialog.getByRole('textbox', { name: 'Search', exact: true }).fill('ownership_roots');
  const result = dialog.locator('.pagefind-ui__result-link').first();
  await expect(result).toBeVisible();
  await result.focus();
  await page.keyboard.press('Enter');
  await expect(page).toHaveURL(/\/docs\/(?:concepts|reference)\//);
  await page.waitForLoadState('load');
  await expect(page.getByRole('dialog')).not.toBeVisible();
  await page.keyboard.press('Control+k');
  await expect(page.getByRole('dialog')).toBeVisible();
  await page.keyboard.press('Escape');
  await expect(page.getByRole('dialog')).not.toBeVisible();
});

test('imported code tabs keep complete samples, keyboard behavior and copying', async ({ page, context }) => {
  await context.grantPermissions(['clipboard-read', 'clipboard-write']);
  await page.goto('/docs/concepts/configuration/');
  const first = page.getByRole('tab', { name: 'fensu.toml', exact: true });
  await first.focus();
  await page.keyboard.press('ArrowRight');
  const second = page.getByRole('tab', { name: 'pyproject.toml', exact: true });
  await expect(second).toHaveAttribute('aria-selected', 'true');
  const panel = page.getByRole('tabpanel', { name: 'pyproject.toml' });
  await expect(panel).toContainText('[tool.fensu.thresholds]');
  await panel.getByRole('button', { name: 'Copy to clipboard' }).click();
  expect(await page.evaluate(() => navigator.clipboard.readText())).toBe('[tool.fensu]\nroots = ["src/my_package"]\ntests = ["tests"]\ntooling = ["scripts"]\nselect = ["FF"]\n\n[tool.fensu.thresholds]\nmax_positional_args = 1');
  await page.goto('/docs/concepts/architecture-model/');
  await page.getByRole('tab', { name: 'After', exact: true }).click();
  await expect(page.getByRole('tabpanel', { name: 'After', exact: true })).toContainText('├── annotation_export/');
});

test('theme choice persists between marketing and docs in both directions', async ({ page }) => {
  await page.emulateMedia({ colorScheme: 'light' });
  await page.goto('/');
  await page.getByRole('button', { name: 'Switch to dark theme' }).click();
  await page.getByRole('navigation', { name: 'Main', exact: true }).getByRole('link', { name: 'Quickstart' }).click();
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark');
  await expect(page.getByRole('button', { name: 'Switch to light theme' })).toHaveAttribute('aria-pressed', 'true');
  await page.setViewportSize({ width: 390, height: 844 });
  await page.getByRole('button', { name: 'Menu', exact: true }).click();
  await page.getByRole('button', { name: 'Switch to light theme' }).click();
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');
  await page.setViewportSize({ width: 1440, height: 1000 });
  await expect(page.getByRole('button', { name: 'Switch to dark theme' })).toHaveAttribute('aria-pressed', 'false');
  await page.getByRole('link', { name: 'Fensu home' }).click();
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');
});

for (const colorScheme of ['light', 'dark']) {
  for (const mobile of [false, true]) {
    test(`docs content, menu and accessibility in ${colorScheme}, ${mobile ? 'mobile' : 'desktop'}`, async ({ browser, baseURL }) => {
      const context = await browser.newContext({
        baseURL, colorScheme,
        ...(mobile ? devices['Pixel 7'] : { viewport: { width: 1440, height: 1000 } }),
      });
      const page = await context.newPage();
      for (const route of ['/docs/quickstart/', '/docs/concepts/configuration/', '/docs/']) {
        await page.goto(route);
        await page.evaluate(() => document.fonts.ready);
        // Expressive Code sets scroll focus after its debounced ResizeObserver.
        await page.waitForFunction(() => [...document.querySelectorAll('.expressive-code pre')].every((block) => block.scrollWidth <= block.clientWidth || block.tabIndex === 0));
        expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), route).toBe(true);
        await expect(page.locator('html')).toHaveAttribute('data-theme', colorScheme);
        const header = await page.locator('.site-header').boundingBox();
        const lockup = await page.locator('.brand-lockup').boundingBox();
        const search = await page.locator('.header .search').boundingBox();
        expect(lockup.y - header.y).toBeGreaterThanOrEqual(12);
        expect(header.y + header.height - lockup.y - lockup.height).toBeGreaterThanOrEqual(12);
        expect(Math.abs(search.y + search.height / 2 - lockup.y - lockup.height / 2)).toBeLessThan(1);
        await expect(page.getByRole('link', { name: 'by SQLStack', exact: true })).toHaveAttribute('href', 'https://sqlstack.io');
        await expect(page.getByRole('link', { name: 'Part of SQLStack', exact: true })).toHaveAttribute('href', 'https://sqlstack.io');
        await expect(page.getByRole('navigation', { name: 'SQLStack tools' }).getByRole('link')).toHaveText(['SQLBuild', 'SQLClean', 'SQLTide', 'Fensu']);
        for (const [name, href] of Object.entries({ SQLBuild: 'https://sqlbuild.com', SQLClean: 'https://sqlclean.io', SQLTide: 'https://sqltide.io/', Fensu: '/' })) {
          await expect(page.getByRole('navigation', { name: 'SQLStack tools' }).getByRole('link', { name, exact: true })).toHaveAttribute('href', href);
        }
        await expect(page.locator('.sqlstack-brand img:visible')).toHaveCount(1);
        const results = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa', 'wcag22aa']).analyze();
        expect(results.violations, route).toEqual([]);
        if (mobile && route.includes('configuration')) {
          const table = page.locator('.sl-markdown-content table').first();
          expect(await table.evaluate((element) => element.scrollWidth > element.clientWidth)).toBe(true);
          await table.focus();
          await page.keyboard.press('ArrowRight');
          await expect.poll(() => table.evaluate((element) => element.scrollLeft)).toBeGreaterThan(0);
        }
      }
      await expect(page.locator('.sqb-frame img:visible')).toHaveCount(1);
      expect(await page.locator('.sqb-frame img:visible').evaluate((image) => image.complete && image.naturalWidth > 0)).toBe(true);
      if (mobile) {
        expect(await page.evaluate(() => matchMedia('(pointer: coarse)').matches)).toBe(true);
        await page.getByRole('button', { name: 'Menu', exact: true }).tap();
        const nav = page.getByRole('navigation', { name: 'Main', exact: true });
        await expect(nav.getByRole('link', { name: 'Quickstart', exact: true })).toBeVisible();
        await nav.getByRole('link', { name: 'Quickstart', exact: true }).tap();
        await expect(page).toHaveURL(/\/docs\/quickstart\/$/);
        await expect(page.getByRole('heading', { level: 1 })).toHaveText('Quickstart');
      }
      await context.close();
    });
  }
}
