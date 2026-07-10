import { chromium } from 'playwright';

async function run() {
  const chunks = [];
  for await (const chunk of process.stdin) {
    chunks.push(chunk);
  }

  const input = JSON.parse(Buffer.concat(chunks).toString('utf8'));
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();

  await page.setContent(input.html ?? '', { waitUntil: 'load' });
  const selector = input.selector ?? 'body';
  const result = await page.$eval(selector, (el) => el.innerText);

  await browser.close();
  process.stdout.write(JSON.stringify({ innerText: result }));
}

run().catch((err) => {
  process.stderr.write(String(err));
  process.exitCode = 1;
});
