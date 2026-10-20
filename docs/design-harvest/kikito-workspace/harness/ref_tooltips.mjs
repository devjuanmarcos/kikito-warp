import { launchChrome } from './cdp.mjs';
import fs from 'node:fs';
const out = process.argv[2];
fs.mkdirSync(out, { recursive: true });
const c = await launchChrome({ profileDir: out + '/chrome-profile' });
await c.navigate('http://127.0.0.1:8765/ui_kits/workspace/index.html');
for (let i = 0; i < 60; i++) { if (await c.evaluate(`!![...document.querySelectorAll('div')].find(d=>d.style.maxWidth==='1400px')`)) break; await new Promise(r => setTimeout(r, 500)); }
await c.evaluate(`(()=>{const fr=[...document.querySelectorAll('div')].find(d=>d.style.maxWidth==='1400px'); fr.setAttribute('data-theme','dark'); localStorage.removeItem('kt-workspace-v1');})()`);
const tiles = JSON.parse(await c.evaluate(`(()=>{const sp=[...document.querySelectorAll('span')].filter(s=>{const r=s.getBoundingClientRect();return Math.abs(r.width-36)<1&&Math.abs(r.height-36)<1&&r.x<120});return JSON.stringify(sp.map(s=>{const r=s.getBoundingClientRect();return {x:r.x+18,y:r.y+18,t:s.textContent}}))})()`));
const K = tiles.find(t => t.t === 'K');
await c.click(K.x, K.y);
await c.click(214, 148);
const park = async () => { await c.move(900, 600); };
await park();
await c.shot(out + '/tt_rest.png');
const targets = {
  home: [57, 84], K: [57, 141], P: [57, 185], F: [57, 229], add: [57, 273], gear: [57, 746],
  header_plus: [318, 41], tab_plus: [955, 41], git: [1235, 41],
};
for (const [name, [x, y]] of Object.entries(targets)) {
  await park();
  await c.move(x, y);
  await new Promise(r => setTimeout(r, 400));
  await c.shot(`${out}/tt_${name}.png`);
}
await park();
await c.move(214, 192);
await c.move(285, 187);
await new Promise(r => setTimeout(r, 400));
await c.shot(out + '/tt_row_plus.png');
c.close();
console.log('done');
