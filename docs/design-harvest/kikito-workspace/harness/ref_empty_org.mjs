import { launchChrome } from './cdp.mjs';
const out = process.argv[2];
const c = await launchChrome({ profileDir: out + '/chrome-profile' });
await c.navigate('http://127.0.0.1:8765/ui_kits/workspace/index.html');
for (let i = 0; i < 60; i++) { if (await c.evaluate(`!![...document.querySelectorAll('div')].find(d=>d.style.maxWidth==='1400px')`)) break; await new Promise(r => setTimeout(r, 500)); }
await c.evaluate(`(()=>{const fr=[...document.querySelectorAll('div')].find(d=>d.style.maxWidth==='1400px'); fr.setAttribute('data-theme','dark'); localStorage.removeItem('kt-workspace-v1');})()`);
await c.click(57, 273);
await new Promise(r => setTimeout(r, 400));
await c.send('Input.insertText', { text: 'Nova' });
await new Promise(r => setTimeout(r, 200));
await c.evaluate(`(()=>{const b=[...document.querySelectorAll('[role=dialog] button')].find(b=>b.textContent.startsWith('Criar'));b.click();})()`);
await new Promise(r => setTimeout(r, 500));
await c.move(900, 700);
await new Promise(r => setTimeout(r, 400));
await c.shot(out + '/empty_org.png');
c.close();
