"""Region-wise pixel parity. usage: compare.py spec.json
spec: [{"name":..., "ref":"a.png", "nat":"b.png", "rbox":[x,y,w,h], "nbox":[x,y,w,h], "tol":10}]
Score = share of pixels whose every RGB channel differs by <= tol (after blending alpha on black)."""
import json, sys
import numpy as np
from PIL import Image

def crop(path, box):
    im = Image.open(path).convert('RGBA')
    bg = Image.new('RGBA', im.size, (0, 0, 0, 255))
    im = Image.alpha_composite(bg, im).convert('RGB')
    x, y, w, h = box
    return np.asarray(im.crop((x, y, x + w, y + h))).astype(int)

spec = json.load(open(sys.argv[1], encoding='utf-8'))
rows = []
for item in spec:
    a = crop(item['ref'], item['rbox'])
    b = crop(item['nat'], item['nbox'])
    tol = item.get('tol', 10)
    ok = (np.abs(a - b) <= tol).all(axis=2)
    if 'ignore' in item:
        ix, iy, iw, ih = item['ignore']
        keep = np.ones(ok.shape, bool); keep[iy:iy+ih, ix:ix+iw] = False
        ok = ok[keep]
    score = ok.mean() * 100
    mad = np.abs(a - b).mean()
    rows.append((item['name'], score, mad))
    if item.get('save'):
        side = np.concatenate([a, b, np.abs(a - b).clip(0, 255) * 4], axis=1).clip(0, 255).astype('uint8')
        Image.fromarray(side).resize((side.shape[1] * 4, side.shape[0] * 4), Image.NEAREST).save(item['save'])
width = max(len(r[0]) for r in rows)
for n, s, m in rows:
    print(f"{n:<{width}}  {s:6.1f}%   mean|diff|={m:5.2f}")
print(f"{'MEDIA':<{width}}  {np.mean([r[1] for r in rows]):6.1f}%")
