import { expect, test } from '@playwright/test';

test('timer persists across reloads', async ({ page }) => {
  await page.goto('/login');
  await page.fill('input[name="email"]', 'timer@example.com');
  await page.click('button[type="submit"]');
  await page.waitForURL('**/dashboard');

  await page.getByRole('button', { name: 'Start Session' }).click();
  await page.waitForTimeout(2200);
  await page.reload();

  const timer = page.getByText(/\d{2}:\d{2}:\d{2}/);
  await expect(timer).toBeVisible();
  const text = await timer.innerText();
  expect(text).not.toBe('00:00:00');
});
