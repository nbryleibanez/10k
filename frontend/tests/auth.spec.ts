import { expect, test } from '@playwright/test';

test('can login via mock auth', async ({ page }) => {
  await page.goto('/login');
  await page.fill('input[name="email"]', 'test@example.com');
  await page.click('button[type="submit"]');
  await page.waitForURL('**/dashboard');
  await expect(page.getByText('Welcome back!')).toBeVisible();
});
