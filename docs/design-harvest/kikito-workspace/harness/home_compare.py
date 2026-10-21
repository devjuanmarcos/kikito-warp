"""Home parity. usage: home_compare.py ref.png nat.png
The kit's Home shows an agent action row ("Ver Codex", "Retomar ...") that the native Home
leaves out by product decision, which pushes the content below the greeting down by ~40px. Each
region is therefore aligned on its own (vertical search up to 48px, horizontal +-3px) and scored
blurred (sigma 1, tol 16). The native capture starts at (1,1) with the main area at x=66; the
kit frame starts at (25,25) with the main area at x=90, so the native guess is ref - 24."""
import sys
import numpy as np
from PIL import Image, ImageFilter

REGIONS = {
    'barra "Início"': (90, 25, 900, 34),
    'logo + titulo': (192, 108, 360, 46),
    'campo (borda + icone + inicio do texto)': (192, 211, 961, 46),
    'titulo da secao': (192, 286, 961, 24),
    'card 1 (moldura + org + nome)': (192, 318, 232, 58),
    'card 1 (rodape)': (192, 395, 232, 32),
    'grade (molduras dos 4 cards)': (192, 318, 961, 110),
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
    for dy in range(-48, 4):
        for dx in range(-3, 4):
            ny, nx = y - 24 + dy, x - 24 + dx
            if ny < 0:
                continue
            b = nat[ny:ny + h, nx:nx + w]
            if b.shape != a.shape:
                continue
            best = max(best, (score(a, b), dx, dy))
    print(f'{name:<42} {best[0]:5.1f}%  desloc=({best[1]},{best[2]})')
