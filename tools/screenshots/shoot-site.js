// Re-shoot site/screens/*.png — the marketing screenshots used by the landing page.
//
// A browser has no notch, so the app lays out from y=0 and the Dynamic Island in
// device.svg would sit on top of the date line. A device screenshot is 393x852 with
// ~59pt of background above the app's content. So each capture is taken at 393x793
// (the layout the app gets on a phone with safe areas) and placed 59px down on a
// 393x852 canvas. Keep TOP at 59 unless device.svg changes.
//
//   tools/screenshots/serve.py target/site 8765   # NOTE: needs the /app/* SPA fallback
//   node tools/screenshots/shoot-site.js
//
// BASE defaults to http://127.0.0.1:8765/app. CHROME and PW let you point at another
// Chromium / playwright-core.
const fs = require('fs');
const path = require('path');

const PW = process.env.PW || path.join(process.env.HOME, '.nvm/versions/node', process.version, 'lib/node_modules/dev-browser/node_modules/playwright-core');
const CHROME = process.env.CHROME || path.join(process.env.HOME, 'Library/Caches/ms-playwright/chromium-1208/chrome-mac-x64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing');
const { chromium } = require(PW);

const BASE = process.env.BASE || 'http://127.0.0.1:8765/app';
const OUT = path.resolve(__dirname, '../../site/screens');
const NS = 'dew.v1.';
const TOP = 59, W = 393, H = 852, HAPP = H - TOP;

// Same seed as shoot.js: onboarded, no What's New sheet, no tips.
const SETTINGS = { onboarded: true, theme: 'system', translation: 'kjv', text_size: 2, verse_numbers: true, resume_reading: true, verse_nav: 'list', link_mode: 'sheet', auto_update_check: false, leader_notes: true, display_name: '', last_seen_version: '0.2.1' };
const PROGRESS = { last_read: { book: 'HEB', chapter: 11, focus: '' }, last_lesson: null, completed_lessons: ['2026-10-11'], highlights: { 'HEB.11.1': 'yellow', 'HEB.11.6': 'green' }, bookmarks: [], seen_tips: ['today.week', 'lesson.links', 'reader.select', 'bible.goto'] };

// The lesson date is the one "This Sunday" points at on the Today screen.
const SHOTS = [
  ['today', '/'],
  ['manual', '/manual'],
  ['lesson', '/manual/2026-11-15'],
  ['reader', '/bible/HEB/11'],
  ['bible', '/bible'],
  ['settings', '/settings'],
];

const BG = { light: '#fbfbfd', dark: '#000000' }; // --app-bg / dark page
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

(async () => {
  const browser = await chromium.launch({ executablePath: CHROME, headless: true });
  const errors = [];

  for (const dark of [false, true]) {
    const ctx = await browser.newContext({
      viewport: { width: W, height: HAPP },
      deviceScaleFactor: 2, isMobile: true, hasTouch: true,
      colorScheme: dark ? 'dark' : 'light', serviceWorkers: 'block',
    });
    const p = await ctx.newPage();
    p.on('pageerror', (e) => errors.push(e.message));
    p.on('console', (m) => { if (m.type() === 'error') errors.push(m.text()); });

    await p.goto(BASE + '/', { waitUntil: 'load' });
    await sleep(2000);
    await p.evaluate(([ns, s, pr]) => {
      localStorage.clear();
      localStorage.setItem(ns + 'settings', JSON.stringify(s));
      localStorage.setItem(ns + 'progress', JSON.stringify(pr));
    }, [NS, SETTINGS, PROGRESS]);

    for (const [name, route] of SHOTS) {
      await p.goto(BASE + route, { waitUntil: 'load' });
      await sleep(2600);
      await p.screenshot({ path: `/tmp/dew-site-${name}${dark ? '-dark' : ''}.png` });
      console.log('captured', name + (dark ? '-dark' : ''));
    }
    await ctx.close();
  }

  // Compose onto the full 393x852 screen: blank status area, then the app.
  const stage = await browser.newContext({ viewport: { width: W, height: H }, deviceScaleFactor: 2 });
  const sp = await stage.newPage();
  for (const dark of [false, true]) {
    for (const [name] of SHOTS) {
      const src = `/tmp/dew-site-${name}${dark ? '-dark' : ''}.png`;
      const b64 = fs.readFileSync(src).toString('base64');
      await sp.setContent(`<!doctype html><body style="margin:0">
        <div style="width:${W}px;height:${H}px;overflow:hidden;background:${dark ? BG.dark : BG.light}">
          <img src="data:image/png;base64,${b64}" width="${W}" height="${HAPP}" style="display:block;margin-top:${TOP}px">
        </div></body></html>`);
      await sp.waitForFunction(() => document.images[0] && document.images[0].complete);
      const file = `${OUT}/${name}${dark ? '-dark' : ''}.png`;
      await sp.screenshot({ path: file, clip: { x: 0, y: 0, width: W, height: H } });
      console.log('wrote', path.basename(file));
    }
  }
  await browser.close();
  if (errors.length) { console.error('PAGE ERRORS:\\n' + errors.join('\\n')); process.exit(1); }
})();
