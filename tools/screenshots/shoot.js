const puppeteer = require('puppeteer-core');
const BASE = process.env.BASE || 'http://127.0.0.1:8765';
const OUT = process.env.OUT || '.';
const NS = 'dew.v1.';
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

async function page(browser, dark) {
  const p = await browser.newPage();
  await p.setViewport({ width: 390, height: 844, deviceScaleFactor: 2, isMobile: true, hasTouch: true });
  await p.emulateMediaFeatures([{ name: 'prefers-color-scheme', value: dark ? 'dark' : 'light' }]);
  p.on('pageerror', (e) => console.log('PAGEERROR', e.message));
  p.on('console', (m) => { if (m.type() === 'error') console.log('CONSOLE', m.text()); });
  return p;
}
async function seed(p, settings, progress) {
  await p.goto(BASE + '/', { waitUntil: 'load' });
  await sleep(1500);
  await p.evaluate((ns, s, pr) => {
    localStorage.clear();
    if (s) localStorage.setItem(ns + 'settings', JSON.stringify(s));
    if (pr) localStorage.setItem(ns + 'progress', JSON.stringify(pr));
  }, NS, settings, progress);
  await sleep(300);
}
async function shot(p, url, name, extra) {
  await p.goto(BASE + url, { waitUntil: 'load' });
  await sleep(2500);
  if (extra) { await extra(p); await sleep(900); }
  await p.screenshot({ path: `${OUT}/${name}.png` });
  console.log('shot', name);
}
async function clickText(p, text) {
  await p.evaluate((t) => {
    const el = [...document.querySelectorAll('button, a')].find((e) => e.textContent.trim().startsWith(t));
    if (el) el.click();
  }, text);
}

(async () => {
  const browser = await puppeteer.launch({ executablePath: '/usr/bin/chromium', args: ['--no-sandbox', '--disable-gpu'] });
  const done = { onboarded: true, theme: 'system', translation: 'kjv', text_size: 2, verse_numbers: true, link_mode: 'sheet', auto_update_check: false, leader_notes: true, display_name: '', last_seen_version: '0.1.0' };
  const progress = { last_read: { book: 'HEB', chapter: 11 }, last_lesson: null, completed_lessons: ['2026-10-11'], highlights: { 'HEB.11.1': 'yellow', 'HEB.11.6': 'green' }, bookmarks: [], seen_tips: ['today.week','lesson.links','reader.select','bible.goto'] };

  // Onboarding (fresh install): welcome, 3 tour pages, 3 setup pages
  let p = await page(browser, false);
  await seed(p, null, null);
  await shot(p, '/', '01-welcome');
  const names = ['02-tour-links', '03-tour-return', '04-tour-highlight', '05-setup-translation', '06-setup-appearance', '07-setup-links'];
  for (const n of names) { await clickText(p, 'Continue'); await sleep(900); await p.screenshot({ path: `${OUT}/${n}.png` }); console.log('shot', n); }
  // Tips (fresh progress, onboarded)
  await seed(p, done, { last_read: null, completed_lessons: [], highlights: {}, bookmarks: [], seen_tips: [] });
  await shot(p, '/', '08-tip-today');
  await shot(p, '/manual/2026-10-18', '09-tip-lesson');
  await shot(p, '/bible/JHN/3', '10-tip-reader');

  for (const dark of [false, true]) {
    const sfx = dark ? '-dark' : '';
    p = await page(browser, dark);
    await seed(p, done, progress);
    await shot(p, '/', '11-today' + sfx);
    await shot(p, '/manual', '12-manual' + sfx);
    await shot(p, '/manual/2026-10-18', '13-lesson' + sfx);
    if (!dark) {
      await shot(p, '/manual/2026-10-18', '14-lesson-discussion', async (pg) => {
        await pg.evaluate(() => { const h = [...document.querySelectorAll('h2')].find((e) => e.textContent.includes('Discussion')); h && window.scrollTo(0, h.getBoundingClientRect().top + window.scrollY - 60); });
      });
      // tap a reference -> preview sheet
      await shot(p, '/manual/2026-10-18', '15-verse-preview-sheet', async (pg) => {
        await pg.evaluate(() => { const b = [...document.querySelectorAll('button')].find((e) => e.textContent.trim().startsWith('Heb 11:1-3,6')); b && b.scrollIntoView({ block: 'center' }); b && b.click(); });
        await sleep(800);
      });
      // Open in Bible -> reader with return pill
      await clickText(p, 'Open in Bible'); await sleep(1800);
      await p.screenshot({ path: `${OUT}/16-reader-return-pill.png` });
      console.log('shot 10');
    }
    await shot(p, '/bible/HEB/11', '17-reader' + sfx);
    await shot(p, '/bible', '18-books' + sfx, async (pg) => { await pg.evaluate(() => window.scrollTo(0, 0)); });
    await shot(p, '/settings', '19-settings' + sfx);
  }
  // Inline drop-down mode
  p = await page(browser, false);
  await seed(p, { ...done, link_mode: 'inline' }, progress);
  await shot(p, '/manual/2026-10-25', '20-inline-dropdown', async (pg) => {
    await pg.evaluate(() => { const b = [...document.querySelectorAll('button')].find((e) => e.textContent.trim().startsWith('Php 1:3-6')); b && b.scrollIntoView({ block: 'start' }); window.scrollBy(0, -120); b && b.click(); });
  });
  await browser.close();
})();
