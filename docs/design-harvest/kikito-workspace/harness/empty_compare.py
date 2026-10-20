"""Empty-project parity. usage: empty_compare.py ref.png nat.png
Each region is aligned independently (the native button is wider because Windows shows
"Ctrl Shift T" where the kit shows the macOS "⌘ T"), then scored blurred (sigma 1, tol 16)."""
import sys
import numpy as np
from PIL import Image, ImageFilter

# (x, y, w, h) in the reference; the main area of both captures is centered on (797, 417).
REGIONS = {
    'fundo (area principal)': (360, 120, 870, 220),
    'texto': (672, 384, 250, 24),
    'botao (borda esquerda + "+ Nova aba")': (716, 412, 102, 40),
}


def load(path):
    im = Image.open(path).convert('RGB').filter(ImageFilter.GaussianBlur(1))
    return np.asarray(im).astype(int)


def score(a, b):
    return (np.abs(a - b) <= 16).all(axis=2).mean() * 100


ref, nat = load(sys.argv[1]), load(sys.argv[2])
for name, (x, y, w, h) in REGIONS.items():
    a = ref[y:y + h, x:x + w]
    best = (0, 0, 0)
    for dy in range(-6, 7):
        for dx in range(-30, 31):
            b = nat[y + dy:y + dy + h, x + dx:x + dx + w]
            best = max(best, (score(a, b), dx, dy))
    print(f'{name:<40} {best[0]:5.1f}%  desloc=({best[1]},{best[2]})')
