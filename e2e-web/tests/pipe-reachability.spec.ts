import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { expect, test } from '@playwright/test';

const stub = pathToFileURL(path.join(__dirname, '..', 'stub', 'pipe.html')).href;

test('pipe: the stub page shows its heading', async ({ page }) => {
  await page.goto(stub);
  await expect(page.getByRole('heading', { name: 'viola browser pipe' })).toBeVisible();
});
