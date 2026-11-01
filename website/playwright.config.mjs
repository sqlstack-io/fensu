import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './tests',
  fullyParallel: true,
  workers: 2,
  use: {
    baseURL: 'http://127.0.0.1:4174',
    ...(process.env.CHROME_PATH ? { launchOptions: { executablePath: process.env.CHROME_PATH } } : {}),
  },
  webServer: {
    command: 'npm run build && python3 scripts/check_links.py dist && python3 -m http.server 4174 --bind 127.0.0.1 --directory dist',
    url: 'http://127.0.0.1:4174',
  },
});
