// Use an existing Playwright installation; the exported viewer has no runtime dependencies.
import assert from 'node:assert/strict';
import { resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
const [modulePath, replayPath, executablePath] = process.argv.slice(2);
assert.ok(modulePath && replayPath, 'usage: node tools/check-viewer.mjs PLAYWRIGHT_MODULE REPLAY.html [CHROMIUM]');
const { chromium } = await import(pathToFileURL(resolve(modulePath)));
const browser = await chromium.launch({ headless:true, executablePath });
try {
  const page = await browser.newPage({ viewport:{width:1440,height:1000} });
  const errors = [];
  const requests = [];
  page.on('pageerror', error => errors.push(error.message));
  page.on('request', request => requests.push(request.url()));
  await page.goto(pathToFileURL(resolve(replayPath)).href);
  const initial = await page.evaluate(() => {
    const data = JSON.parse(document.getElementById('trace-data').textContent);
    return {manifest:data.rows[0].manifest, symbols:data.symbols, rows:data.rows, frames:data.rows.filter(row=>row.kind==='checkpoint')};
  });
  for (let r=0; r<12; r++) {
    assert.equal(await page.locator('#registers strong').nth(r).textContent(), initial.manifest.initial_registers[r]);
  }
  await page.locator('#registers button').nth(1).click();
  assert.match(await page.locator('#detail').textContent(), /Effective engine values/);
  assert.ok((await page.locator('#instructions button').first().textContent())
    .includes(initial.manifest.instructions[0].map(n=>initial.symbols[n]).join(' / ')));
  await page.locator('#group').selectOption('all');
  assert.equal(await page.locator('#board .card').count(), initial.frames[0].cards.filter(card=>card.zone==='Battlefield').length);
  if (initial.frames.length > 1) {
    await page.locator('#next').click();
    assert.equal(await page.locator('#position').inputValue(), '1');
    await page.locator('#back').click();
    assert.equal(await page.locator('#position').inputValue(), '0');
    const firstPhase = initial.frames.findIndex(frame=>frame.turn!==initial.frames[0].turn || frame.phase!==initial.frames[0].phase);
    await page.locator('#step').selectOption('phase');
    await page.locator('#next').click();
    assert.equal(await page.locator('#position').inputValue(), String(firstPhase));
    await page.locator('#position').evaluate(slider=>{slider.value='0';slider.dispatchEvent(new Event('input'));});
    await page.locator('#step').selectOption('checkpoint');
    await page.locator('#speed').selectOption('250');
    await page.locator('#play').click();
    await page.waitForFunction(()=>Number(document.getElementById('position').value)>0);
    if (await page.locator('#play').textContent()==='Pause') await page.locator('#play').click();
    assert.equal(await page.locator('#play').textContent(), 'Play');
    await page.locator('#position').evaluate((slider,max) => { slider.value=max; slider.dispatchEvent(new Event('input')); }, String(initial.frames.length-1));
    const final = initial.frames.at(-1);
    for (let r=0; r<12; r++) {
      const card = final.cards.find(card=>card.id===`r${r}`);
      assert.equal(await page.locator('#registers strong').nth(r).textContent(), String(card.counters.P1P1 || 0));
    }
  }
  const caused = initial.rows.find(row=>row.parent!==undefined && row.parent!==null);
  if (caused) {
    const boundary = initial.frames.findLastIndex(frame=>frame.id<caused.id);
    await page.locator('#position').evaluate((slider,value)=>{slider.value=value;slider.dispatchEvent(new Event('input'));}, String(boundary));
    await page.locator('#events .event').filter({hasText:new RegExp(`^${caused.id} ·`)}).click();
    assert.match(await page.locator('#detail strong').textContent(), new RegExp(`^Event ${caused.id} ·`));
    await page.locator('#cause button').click();
    assert.match(await page.locator('#detail strong').textContent(), new RegExp(`^Event ${caused.parent} ·`));
    await page.locator('#registers button').nth(1).click();
    assert.equal(await page.locator('#cause button').count(), 0);
  }
  assert.deepEqual(errors, []);
  assert.ok(requests.every(url=>url.startsWith('file:')), 'replay must not require network requests');
  await page.locator('#position').evaluate(slider=>{slider.value='0';slider.dispatchEvent(new Event('input'));});
  await page.locator('#group').selectOption('focus');
  await page.screenshot({path:resolve(replayPath)+'.png',fullPage:true});
  console.log(`offline viewer: ${initial.frames.length} checkpoints, decoded counters, full board, phase stepping, playback, causal links, no browser errors`);
} finally {
  await browser.close();
}
