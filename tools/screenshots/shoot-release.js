const puppeteer = require('puppeteer-core');
const fs = require('fs');
const BASE = 'http://127.0.0.1:8765';
const OUT = process.env.OUT || '.';
const PUB = process.env.PUB;
const NS = 'dew.v1.';
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const done = (v) => ({ onboarded: true, theme: 'system', translation: 'kjv', text_size: 2, verse_numbers: true, link_mode: 'sheet', auto_update_check: true, leader_notes: true, display_name: '', last_seen_version: v });
const progress = { seen_tips: ['today.week','lesson.links','reader.select','bible.goto'] };
(async () => {
  const b = await puppeteer.launch({ executablePath: '/usr/bin/chromium', args: ['--no-sandbox', '--disable-gpu'] });
  const p = await b.newPage();
  await p.setViewport({ width: 390, height: 844, deviceScaleFactor: 2, isMobile: true, hasTouch: true });
  p.on('pageerror', (e) => console.log('PAGEERROR', e.message));
  const seed = async (s) => { await p.goto(BASE + '/'); await sleep(1500); await p.evaluate((ns, s, pr) => { localStorage.clear(); localStorage.setItem(ns + 'settings', JSON.stringify(s)); localStorage.setItem(ns + 'progress', JSON.stringify(pr)); }, NS, s, progress); };
  const shot = async (url, name, scroll) => { await p.goto(BASE + url); await sleep(3000); if (scroll) { await p.evaluate((y) => window.scrollTo(0, y), scroll); await sleep(500); } await p.screenshot({ path: `${OUT}/${name}.png` }); console.log('shot', name); };

  const orig = fs.readFileSync(PUB + '/version.json', 'utf8');
  await seed(done('0.0.9'));
  await shot('/', '21-whats-new');
  await seed(done('0.1.0'));
  await shot('/settings/update', '22-software-update-current');
  await shot('/settings/acknowledgements', '23-credits');
  // Pretend CI published 0.2.0
  fs.writeFileSync(PUB + '/version.json', JSON.stringify({ version: '0.2.0', notes: '', published_at: '2026-11-01T09:00:00Z',
    highlights: ['Lessons for November, including the new Unit 5', 'Search across every lesson and the Bible', 'Reading plans for the Daily Dew Drops', 'Faster start-up on older phones'], downloads: {} }));
  await shot('/settings', '24-settings-update-badge', 900);
  await shot('/settings/update', '25-software-update-available');
  fs.writeFileSync(PUB + '/version.json', orig);
  await b.close();
})();
