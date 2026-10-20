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
await c.move(900, 600);
await c.shot(out + '/menu_rest.png');
// right-click on K tile
await c.move(K.x, K.y);
await c.mouse('mousePressed', K.x, K.y, { button: 'right', buttons: 2 });
await c.mouse('mouseReleased', K.x, K.y, { button: 'right', buttons: 2 });
await new Promise(r => setTimeout(r, 400));
await c.move(900, 600);
await c.shot(out + '/menu_org.png');
await c.move(K.x + 40, K.y + 20);
await c.shot(out + '/menu_org_hover1.png');
await c.move(K.x + 40, K.y + 52);
await c.shot(out + '/menu_org_hover2.png');
c.close();
console.log('done');
