"""Empty-organization parity. usage: org_compare.py ref.png nat.png
Regions are paired by their own anchor: the header title is left-aligned after the sidebar, the
search pill is right-aligned (the native bar reserves 138px for the window controls), and the
empty prompt is centered on (797, 417) in both captures. Scored blurred (sigma 1, tol 16) with a
+-3px search to absorb text width differences."""
import sys
import numpy as np
from PIL import Image, ImageFilter

# name: (ref box (x, y, w, h), native origin (x, y))
PAIRS = {
    'sidebar (vazio)': ((90, 25, 249, 160), (66, 1)),
    'cabecalho titulo': ((339, 25, 300, 34), (315, 1)),
    'cabecalho busca': ((999, 25, 256, 34), (885, 1)),
    'centro texto': ((640, 380, 314, 30), (640, 380)),
    'centro botao': ((725, 410, 144, 44), (725, 410)),
    'fundo': ((360, 120, 870, 220), (340, 120)),
}


def load(path):
    im = Image.open(path).convert('RGB').filter(ImageFilter.GaussianBlur(1))
    return np.asarray(im).astype(int)


def score(a, b):
    return (np.abs(a - b) <= 16).all(axis=2).mean() * 100


ref, nat = load(sys.argv[1]), load(sys.argv[2])
for name, ((x, y, w, h), (nx, ny)) in PAIRS.items():
    a = ref[y:y + h, x:x + w]
    best = (0, 0, 0)
    for dy in range(-3, 4):
        for dx in range(-3, 4):
            if ny + dy < 0:
                continue
            b = nat[ny + dy:ny + dy + h, nx + dx:nx + dx + w]
            best = max(best, (score(a, b), dx, dy))
    print(f'{name:<20} {best[0]:5.1f}%  desloc=({best[1]},{best[2]})')
