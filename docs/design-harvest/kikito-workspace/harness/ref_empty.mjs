import { launchChrome } from './cdp.mjs';
const out = process.argv[2];
const c = await launchChrome({ profileDir: out + '/chrome-profile' });
await c.navigate('http://127.0.0.1:8765/ui_kits/workspace/index.html');
for (let i = 0; i < 60; i++) { if (await c.evaluate(`!![...document.querySelectorAll('div')].find(d=>d.style.maxWidth==='1400px')`)) break; await new Promise(r => setTimeout(r, 500)); }
await c.evaluate(`(()=>{const fr=[...document.querySelectorAll('div')].find(d=>d.style.maxWidth==='1400px'); fr.setAttribute('data-theme','dark'); localStorage.removeItem('kt-workspace-v1');})()`);
const tiles = JSON.parse(await c.evaluate(`(()=>{const sp=[...document.querySelectorAll('span')].filter(s=>{const r=s.getBoundingClientRect();return Math.abs(r.width-36)<1&&Math.abs(r.height-36)<1&&r.x<120});return JSON.stringify(sp.map(s=>{const r=s.getBoundingClientRect();return {x:r.x+18,y:r.y+18,t:s.textContent}}))})()`));
const K = tiles.find(t => t.t === 'K');
await c.click(K.x, K.y); await c.click(214, 238); await c.move(900, 600);
for (let i = 0; i < 5; i++) {
  const n = await c.evaluate(`(()=>{const x=[...document.querySelectorAll('span')].find(s=>s.style.position==='absolute'&&s.style.right==='4px'&&s.getBoundingClientRect().y<60);if(!x)return 0;x.click();return 1;})()`);
  if (!n) break; await new Promise(r => setTimeout(r, 300));
}
await c.move(900, 700);
await new Promise(r => setTimeout(r, 400));
await c.shot(out + '/empty_project.png');
c.close();
