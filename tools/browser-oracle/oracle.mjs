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
  const result = await page.$eval(selector, (el) => ({
    innerText: el.innerText,
    outerText: el.outerText,
    textContent: el.textContent ?? '',
  }));

  await browser.close();
  process.stdout.write(JSON.stringify(result));
}

run().catch((err) => {
  process.stderr.write(String(err));
  process.exitCode = 1;
});
