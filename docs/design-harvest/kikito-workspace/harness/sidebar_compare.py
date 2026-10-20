"""Sidebar parity. usage: sidebar_compare.py ref.png nat.png [save_dir]
Reference frame content starts at (25,25) with the sidebar at x=90; native content starts at
(1,1) with the sidebar at x=66. Top regions align to the top, the footer aligns to the bottom."""
import sys
import numpy as np
from PIL import Image, ImageFilter

REF_X, REF_TOP, REF_BOTTOM = 90, 25, 775
NAT_X, NAT_TOP, NAT_BOTTOM = 66, 1, 798
W = 249
TOP_REGIONS = {
    'cabecalho': (0, 34),
    'busca': (34, 46),
    'titulo': (80, 22),
    'linhas': (102, 140),
}
FOOTER = 41


def load(path, blur=False):
    im = Image.open(path).convert('RGB')
    if blur:
        im = im.filter(ImageFilter.GaussianBlur(1))
    return np.asarray(im).astype(int)


def score(a, b, tol):
    return (np.abs(a - b) <= tol).all(axis=2).mean() * 100


ref, nat = load(sys.argv[1]), load(sys.argv[2])
refb, natb = load(sys.argv[1], True), load(sys.argv[2], True)
pairs = {name: (REF_TOP + y, NAT_TOP + y, h) for name, (y, h) in TOP_REGIONS.items()}
pairs['rodape'] = (REF_BOTTOM - FOOTER, NAT_BOTTOM - FOOTER, FOOTER)
totals = []
for name, (ry, ny, h) in pairs.items():
    a = ref[ry:ry + h, REF_X:REF_X + W]
    b = nat[ny:ny + h, NAT_X:NAT_X + W]
    raw = score(a, b, 10)
    blurred = 0
    for sy in range(-2, 3):
        for sx in range(-2, 3):
            ab = refb[ry:ry + h, REF_X:REF_X + W]
            if ny + sy < 0 or ny + sy + h > natb.shape[0]:
                continue
            bb = natb[ny + sy:ny + sy + h, NAT_X + sx:NAT_X + sx + W]
            blurred = max(blurred, score(ab, bb, 16))
    totals.append((blurred, h))
    print(f'{name:<10} bruto {raw:5.1f}%   desfoque {blurred:5.1f}%')
    if len(sys.argv) > 3:
        side = np.concatenate([a, b, (np.abs(a - b) * 4).clip(0, 255)], axis=1).astype('uint8')
        Image.fromarray(side).resize((side.shape[1] * 2, side.shape[0] * 2), Image.NEAREST).save(
            f'{sys.argv[3]}/sb_{name}.png')
weighted = sum(s * h for s, h in totals) / sum(h for _, h in totals)
print(f'TOTAL      desfoque {weighted:5.1f}%')
