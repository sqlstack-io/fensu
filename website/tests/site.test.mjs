import { test, expect, devices } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test('navigation, keyboard focus and command copying work', async ({ page, context }) => {
  await context.grantPermissions(['clipboard-read', 'clipboard-write']);
  await page.goto('/');
  const mainNav = page.getByRole('navigation', { name: 'Main', exact: true });
  await expect(mainNav.getByRole('link')).toHaveText(['How it works', 'Docs', 'Quickstart', '']);
  const github = mainNav.getByRole('link', { name: 'Fensu on GitHub' });
  await expect(github).toHaveAttribute('href', 'https://github.com/sqlstack-io/fensu');
  await expect(mainNav.getByRole('link', { name: 'Quickstart' })).toHaveAttribute('href', '/docs/quickstart/');
  await page.keyboard.press('Tab');
  await expect(page.getByRole('link', { name: 'Skip to content' })).toBeFocused();
  for (let index = 0; index < 6; index++) await page.keyboard.press('Tab');
  await expect(github).toBeFocused();
  expect(await github.evaluate((element) => element.matches(':focus-visible') && getComputedStyle(element).outlineStyle === 'solid')).toBe(true);
  await mainNav.getByRole('link', { name: 'How it works' }).click();
  await expect(page).toHaveURL(/#how$/);
  const heading = page.getByRole('heading', { name: 'Adopt. Enforce. Understand.' });
  expect((await heading.boundingBox()).y).toBeGreaterThan(60);
  for (const [label, command] of [
    ['Copy install command', 'pip install fensu'],
    ['Copy initialization command', 'fensu init --yes'],
    ['Copy check command', 'fensu check'],
    ['Copy Python call-map command', 'fensu map run_plan --depth 3'],
    ['Copy duplicate-detection command', 'fensu dupes --since origin/main'],
    ['Copy agent-guidance command', 'fensu skills'],
  ]) {
    await page.getByRole('button', { name: label, exact: true }).click();
    expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(command);
    await expect(page.getByRole('status')).toContainText(command);
  }
});

for (const colorScheme of ['light', 'dark']) {
  for (const mobile of [false, true]) {
    test(`readable, responsive and accessible in ${colorScheme}, ${mobile ? 'mobile' : 'desktop'}`, async ({ browser, baseURL }) => {
      const context = await browser.newContext({
        baseURL,
        ...(mobile ? devices['Pixel 7'] : { viewport: { width: 1440, height: 1000 } }),
        colorScheme,
      });
      const page = await context.newPage();
      await page.goto('/');
      await page.evaluate(() => document.fonts.ready);
      await expect(page.locator('html')).toHaveAttribute('data-theme', colorScheme);
      await expect(page.getByRole('heading', { name: 'Turn review comments into rules.', exact: true })).toBeVisible();
      await expect(page.getByText('Trace proven Python calls. Mark unresolved seams.')).toBeVisible();
      await expect(page.getByText(/Advisory, not a failing gate/)).toBeVisible();
      expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
      expect(await page.locator('.brand img:visible').count()).toBe(1);
      if (mobile) expect(await page.evaluate(() => matchMedia('(pointer: coarse)').matches)).toBe(true);
      const header = await page.locator('.header-inner').boundingBox();
      const lockup = await page.locator('.brand-lockup').boundingBox();
      const navigation = await page.getByRole('navigation', { name: 'Main', exact: true }).boundingBox();
      expect(lockup.y - header.y).toBeGreaterThanOrEqual(12);
      expect(header.y + header.height - lockup.y - lockup.height).toBeGreaterThanOrEqual(12);
      expect(Math.abs(navigation.y + navigation.height / 2 - lockup.y - lockup.height / 2)).toBeLessThan(1);
      await expect(page.getByRole('link', { name: 'by SQLStack', exact: true })).toHaveAttribute('href', 'https://sqlstack.io');
      await expect(page.getByRole('link', { name: 'Part of SQLStack', exact: true })).toHaveAttribute('href', 'https://sqlstack.io');
      await expect(page.getByRole('navigation', { name: 'SQLStack tools' }).getByRole('link')).toHaveText(['SQLBuild', 'SQLClean', 'SQLTide', 'Fensu']);
      await expect(page.locator('.sqlstack-brand img:visible')).toHaveCount(1);
      const result = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa', 'wcag22aa']).analyze();
      expect(result.violations).toEqual([]);
      await context.close();
    });
  }
}

test('theme selection updates the whole page and persists', async ({ page }) => {
  await page.emulateMedia({ colorScheme: 'dark' });
  await page.goto('/');
  const darkBackground = await page.locator('body').evaluate((element) => getComputedStyle(element).backgroundColor);
  await page.getByRole('button', { name: 'Switch to light theme' }).click();
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');
  expect(await page.locator('body').evaluate((element) => getComputedStyle(element).backgroundColor)).not.toBe(darkBackground);
  await page.reload();
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');
  await page.getByRole('button', { name: 'Switch to dark theme' }).focus();
  await page.keyboard.press('Enter');
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark');
  await expect(page.getByRole('button', { name: 'Switch to light theme' })).toHaveAttribute('aria-pressed', 'true');
});

test('blocked storage and clipboard do not break theme or misreport copying', async ({ page }) => {
  await page.addInitScript(() => {
    Object.defineProperty(window, 'localStorage', { get() { throw new Error('Unavailable'); } });
    Object.defineProperty(navigator, 'clipboard', { value: { writeText: () => Promise.reject(new Error('Denied')) } });
  });
  await page.goto('/');
  await page.getByRole('button', { name: 'Switch to dark theme' }).click();
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark');
  await page.getByRole('button', { name: 'Copy install command' }).click();
  await expect(page.getByRole('status')).toHaveText('Copy unavailable. Select and copy the command text.');
  await expect(page.getByRole('button', { name: 'Copy install command' })).toHaveText('Copy');
});

test('content and links remain available without JavaScript', async ({ browser, baseURL }) => {
  const context = await browser.newContext({ javaScriptEnabled: false, baseURL });
  const page = await context.newPage();
  await page.goto('/');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Keep your codebase in shape.');
  await expect(page.getByRole('link', { name: 'Get started', exact: true }).first()).toHaveAttribute('href', '/docs/quickstart/');
  await expect(page.getByRole('button', { name: 'Copy install command' })).toBeHidden();
  await context.close();
});

test('built metadata, assets and all internal anchors resolve', async ({ page, request, baseURL }) => {
  const response = await page.goto('/');
  expect((await response.text())).not.toContain('\u2014');
  const resources = await page.locator('img[src], script[src], link[href]').evaluateAll((elements) => elements.map((element) => element.src || element.href));
  for (const url of new Set(resources)) {
    if (url.startsWith(baseURL)) expect((await request.get(url)).ok(), url).toBe(true);
  }
  for (const href of await page.locator('a[href^="#"]').evaluateAll((links) => links.map((link) => link.getAttribute('href')))) {
    await expect(page.locator(href)).toHaveCount(1);
  }
  for (const path of ['/', '/docs/quickstart/']) {
    await page.goto(path);
    await expect(page.locator('link[rel="canonical"]')).toHaveAttribute('href', `https://fensu.io${path}`);
    await expect(page.locator('meta[property="og:url"]')).toHaveAttribute('content', `https://fensu.io${path}`);
    await expect(page.locator('meta[property="og:image"]')).toHaveAttribute('content', 'https://fensu.io/assets/og-fensu-social.png');
    await expect(page.locator('meta[name="twitter:image"]')).toHaveAttribute('content', 'https://fensu.io/assets/og-fensu-social.png');
  }
  const sharingImage = await request.get('/assets/og-fensu-social.png');
  expect(sharingImage.ok()).toBe(true);
  expect(sharingImage.headers()['content-type']).toContain('image/png');
  const { default: sharp } = await import('sharp');
  expect(await sharp(await sharingImage.body()).metadata()).toMatchObject({ width: 1200, height: 630, hasAlpha: false });
  expect(await (await request.get('/sitemap-index.xml')).text()).toContain('<loc>https://fensu.io/sitemap-0.xml</loc>');
  expect(await (await request.get('/sitemap-0.xml')).text()).toContain('<loc>https://fensu.io/docs/quickstart/</loc>');
});
