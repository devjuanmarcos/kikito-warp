import { launchChrome } from './cdp.mjs';
const out = process.argv[2];
const c = await launchChrome({ profileDir: out + '/chrome-profile' });
await c.navigate('http://127.0.0.1:8765/ui_kits/workspace/index.html');
for (let i = 0; i < 60; i++) { if (await c.evaluate(`!![...document.querySelectorAll('div')].find(d=>d.style.maxWidth==='1400px')`)) break; await new Promise(r => setTimeout(r, 500)); }
await c.evaluate(`(()=>{const fr=[...document.querySelectorAll('div')].find(d=>d.style.maxWidth==='1400px'); fr.setAttribute('data-theme','dark'); localStorage.removeItem('kt-workspace-v1');})()`);
await c.navigate('http://127.0.0.1:8765/ui_kits/workspace/index.html');
for (let i = 0; i < 60; i++) { if (await c.evaluate(`!![...document.querySelectorAll('div')].find(d=>d.style.maxWidth==='1400px')`)) break; await new Promise(r => setTimeout(r, 500)); }
await c.evaluate(`(()=>{const fr=[...document.querySelectorAll('div')].find(d=>d.style.maxWidth==='1400px'); fr.setAttribute('data-theme','dark'); localStorage.removeItem('kt-workspace-v1');})()`);
const tiles = JSON.parse(await c.evaluate(`(()=>{const sp=[...document.querySelectorAll('span')].filter(s=>{const r=s.getBoundingClientRect();return Math.abs(r.width-36)<1&&Math.abs(r.height-36)<1&&r.x<120});return JSON.stringify(sp.map(s=>{const r=s.getBoundingClientRect();return {x:r.x+18,y:r.y+18,t:s.textContent}}))})()`));
const K = tiles.find(t => t.t === 'K');
await c.click(K.x, K.y); await c.click(214, 148); await c.move(900, 600);
await c.click(57, 273);
await new Promise(r => setTimeout(r, 400));
await c.evaluate(`(async()=>{const b=await (await fetch('/assets/logo/kikito-terminal-icon.png')).blob();const f=new File([b],'logo.png',{type:'image/png'});const i=document.querySelector('input[type=file]');const dt=new DataTransfer();dt.items.add(f);i.files=dt.files;i.dispatchEvent(new Event('change',{bubbles:true}));})()`);
await new Promise(r => setTimeout(r, 1200));
await c.move(900, 700);
await new Promise(r => setTimeout(r, 300));
await c.shot(out + '/crop.png');
const rects = await c.evaluate(`JSON.stringify([...document.querySelectorAll('[role=dialog], [role=dialog] button')].map(e=>{const r=e.getBoundingClientRect();return {t:(e.textContent||'').trim().slice(0,20),x:r.x,y:r.y,w:r.width,h:r.height}}))`);
console.log(rects);
c.close();
