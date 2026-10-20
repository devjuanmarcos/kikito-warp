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
// remove the tab bar and sidebar demo state differences: select first project row
await c.click(214, 148);
const park = () => c.move(900, 600);
const states = {
  rest: async () => { await park(); },
  hover_header_plus: async () => { await c.move(318, 41); },
  press_header_plus: async () => { await c.move(318, 41); await c.mouse('mousePressed', 318, 41); await new Promise(r => setTimeout(r, 200)); },
  hover_footer: async () => { await c.move(214, 756); },
  press_footer: async () => { await c.move(214, 756); await c.mouse('mousePressed', 214, 756); await new Promise(r => setTimeout(r, 200)); },
  hover_tab_plus: async () => { await c.move(955, 41); },
  hover_git: async () => { await c.move(1235, 41); },
  hover_row: async () => { await c.move(214, 192); },
};
for (const [name, fn] of Object.entries(states)) {
  await park();
  await c.mouse('mouseReleased', 900, 600);
  await fn();
  await new Promise(r => setTimeout(r, 300));
  await c.shot(`${out}/ref_${name}.png`);
}
c.close();
console.log('done');
