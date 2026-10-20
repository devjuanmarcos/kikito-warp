// Minimal Chrome DevTools Protocol client (Node 24 has global WebSocket + fetch).
import { spawn } from 'node:child_process';
import fs from 'node:fs';

export async function launchChrome({ port = 9333, profileDir, width = 1280, height = 800 }) {
  const chrome = spawn('C:/Program Files/Google/Chrome/Application/chrome.exe', [
    `--remote-debugging-port=${port}`,
    `--user-data-dir=${profileDir}`,
    '--headless=new', '--hide-scrollbars', '--force-device-scale-factor=1',
    `--window-size=${width},${height}`, '--no-first-run', '--disable-gpu', 'about:blank',
  ], { stdio: 'ignore' });
  let targets;
  for (let i = 0; i < 50; i++) {
    try { targets = await (await fetch(`http://127.0.0.1:${port}/json`)).json(); if (targets.length) break; } catch {}
    await new Promise(r => setTimeout(r, 200));
  }
  const page = targets.find(t => t.type === 'page');
  const ws = new WebSocket(page.webSocketDebuggerUrl);
  await new Promise(r => ws.addEventListener('open', r));
  let id = 0; const pending = new Map();
  ws.addEventListener('message', e => { const m = JSON.parse(e.data); if (m.id && pending.has(m.id)) { pending.get(m.id)(m); pending.delete(m.id); } });
  const send = (method, params = {}) => new Promise((res, rej) => { const i = ++id; pending.set(i, m => m.error ? rej(new Error(JSON.stringify(m.error))) : res(m.result)); ws.send(JSON.stringify({ id: i, method, params })); });
  await send('Page.enable'); await send('Runtime.enable');
  await send('Emulation.setDeviceMetricsOverride', { width, height, deviceScaleFactor: 1, mobile: false });
  const evaluate = async (expression) => { const r = await send('Runtime.evaluate', { expression, returnByValue: true, awaitPromise: true }); if (r.exceptionDetails) throw new Error(JSON.stringify(r.exceptionDetails)); return r.result.value; };
  const shot = async (file, clip) => { const r = await send('Page.captureScreenshot', { format: 'png', ...(clip ? { clip: { ...clip, scale: 1 } } : {}) }); fs.writeFileSync(file, Buffer.from(r.data, 'base64')); };
  const mouse = (type, x, y, extra = {}) => send('Input.dispatchMouseEvent', { type, x, y, button: 'left', clickCount: 1, ...extra });
  const move = async (x, y) => { await mouse('mouseMoved', x, y, { button: 'none', clickCount: 0 }); await new Promise(r => setTimeout(r, 250)); };
  const click = async (x, y) => { await move(x, y); await mouse('mousePressed', x, y); await mouse('mouseReleased', x, y); await new Promise(r => setTimeout(r, 300)); };
  const navigate = async (url) => { await send('Page.navigate', { url }); await new Promise(r => setTimeout(r, 2500)); };
  const close = () => { try { ws.close(); } catch {} chrome.kill(); };
  return { send, evaluate, shot, move, click, mouse, navigate, close };
}
