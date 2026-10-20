import sys, json
import numpy as np
from PIL import Image

def rgb(p):
    im = Image.open(p).convert('RGBA'); bg = Image.new('RGBA', im.size, (0, 0, 0, 255))
    return np.asarray(Image.alpha_composite(bg, im).convert('RGB')).astype(int)

def find(path, win, fill, tol=4):
    a = rgb(path); x0, y0, x1, y1 = win
    sub = a[y0:y1, x0:x1]
    m = (np.abs(sub - np.array(fill)) <= tol).all(axis=2)
    rows = np.where(m.sum(axis=1) > 6)[0]; cols = np.where(m.sum(axis=0) > 3)[0]
    if len(rows) == 0:
        return None
    return (x0 + int(cols.min()), y0 + int(rows.min()), int(cols.max() - cols.min() + 1), int(rows.max() - rows.min() + 1))

cases = json.load(open(sys.argv[1], encoding='utf-8'))
ref_fill = [155, 155, 155]
print('fill ref', ref_fill)
tot = []
for c in cases:
    r = find(c['ref'], c['rwin'], ref_fill)
    n = find(c['nat'], c['nwin'], ref_fill, tol=12)
    line = f"{c['name']:<22} ref={r} nat={n}"
    if r and n:
        dx = (n[0] - c['ncenter'][0]) - (r[0] - c['rcenter'][0]); dy = (n[1] - c['ncenter'][1]) - (r[1] - c['rcenter'][1])
        line += f"  desloc(dx,dy)=({dx},{dy}) altura {n[3]} vs {r[3]}"
        if c.get('pixels') and r[2] == n[2] and r[3] == n[3]:
            a = rgb(c['ref'])[r[1]:r[1]+r[3], r[0]:r[0]+r[2]]; b = rgb(c['nat'])[n[1]:n[1]+n[3], n[0]:n[0]+n[2]]
            sc = (np.abs(a - b) <= 24).all(axis=2).mean() * 100
            line += f"  pixels {sc:.1f}%"; tot.append(sc)
    print(line)
