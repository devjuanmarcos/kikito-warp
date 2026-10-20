import { launchChrome } from './cdp.mjs';
const out = process.argv[2];
const c = await launchChrome({ profileDir: out + '/chrome-profile' });
await c.navigate('http://127.0.0.1:8765/ui_kits/workspace/index.html');
for (let i = 0; i < 60; i++) { if (await c.evaluate(`!![...document.querySelectorAll('div')].find(d=>d.style.maxWidth==='1400px')`)) break; await new Promise(r => setTimeout(r, 500)); }
await c.evaluate(`(()=>{const fr=[...document.querySelectorAll('div')].find(d=>d.style.maxWidth==='1400px'); fr.setAttribute('data-theme','dark'); localStorage.removeItem('kt-workspace-v1');})()`);
const tiles = JSON.parse(await c.evaluate(`(()=>{const sp=[...document.querySelectorAll('span')].filter(s=>{const r=s.getBoundingClientRect();return Math.abs(r.width-36)<1&&Math.abs(r.height-36)<1&&r.x<120});return JSON.stringify(sp.map(s=>{const r=s.getBoundingClientRect();return {x:r.x+18,y:r.y+18,t:s.textContent}}))})()`));
const K = tiles.find(t => t.t === 'K');
await c.click(K.x, K.y); await c.click(214, 148); await c.move(900, 600);
await c.click(318, 41);
await new Promise(r => setTimeout(r, 400));
await c.move(900, 700);
await c.shot(out + '/modal_project.png');
const rects = await c.evaluate(`JSON.stringify([...document.querySelectorAll('[role=dialog], [role=dialog] button, [role=dialog] input, [role=dialog] label')].map(e=>{const r=e.getBoundingClientRect();return {tag:e.tagName,t:(e.textContent||'').trim().slice(0,24),x:r.x,y:r.y,w:r.width,h:r.height}}))`);
console.log(rects);
await c.click(640, 500);
c.close();
