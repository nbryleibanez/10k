import { expect, test } from '@playwright/test';

test('landing page has hero copy', async ({ page }) => {
  await page.goto('/');
  await expect(page.getByText('Track every deliberate hour')).toBeVisible();
});
