import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: 'tests',
  outputDir: 'test-results',
  retries: 0,
  forbidOnly: true,
  reporter: [
    ['json', { outputFile: 'pw.json' }],
    ['junit', { outputFile: 'pw-junit.xml' }],
  ],
  projects: [
    {
      name: 'chromium',
      use: { headless: true },
    },
  ],
});
