"""Source control panel parity. usage: git_compare.py ref.png nat.png
The kit panel spans x 943..1255 (34% of its main area); the native one is 34% of a wider area,
so regions are anchored to the panel's left or right edge and to its top or bottom, then scored
blurred (sigma 1, tol 16) with a +-3px search."""
import sys
import numpy as np
from PIL import Image, ImageFilter

REF = dict(left=943, right=1255, top=59, bottom=776)


def native_bounds(img):
    """Finds the native panel: its left border is the outline column right of the main area."""
    a = np.asarray(img.convert('L')).astype(int)
    row = a[300]
    for x in range(1279 - 300, 1279 - 420, -1):
        if row[x] > 30 and row[x - 1] < 15 and row[x + 1] < 30:
            return dict(left=x, right=1279, top=35, bottom=799)
    return dict(left=951, right=1279, top=35, bottom=799)


# name: (anchor_x, x, anchor_y, y, w, h); x/y are offsets from the chosen edges.
REGIONS = {
    'cabecalho (titulo)': ('left', 0, 'top', 0, 200, 34),
    'cabecalho (fechar)': ('right', -36, 'top', 0, 36, 34),
    'ALTERACOES (titulo)': ('left', 0, 'top', 35, 110, 26),
    'ALTERACOES (branch+acoes)': ('right', -126, 'top', 35, 126, 26),
    'mensagem (inicio)': ('left', 0, 'top', 61, 200, 54),
    'commit (botao)': ('left', 0, 'top', 115, 150, 34),
    'lista (cabecalho+arquivos)': ('left', 0, 'top', 152, 200, 120),
    'lista (letras de status)': ('right', -30, 'top', 152, 30, 120),
    'GRAFO (titulo)': ('left', 0, 'top', 281, 90, 26),
    'grafo (linhas)': ('left', 0, 'top', 307, 200, 50),
    'grafo (pilula HEAD)': ('right', -60, 'top', 307, 60, 26),
    'rodape (branch)': ('left', 0, 'bottom', -28, 150, 28),
    'rodape (Publicar)': ('right', -94, 'bottom', -28, 94, 28),
}


def load(path):
    im = Image.open(path).convert('RGB')
    return im, np.asarray(im.filter(ImageFilter.GaussianBlur(1))).astype(int)


def origin(bounds, ax, x, ay, y):
    return bounds[ax] + x, bounds[ay] + y


ref_img, ref = load(sys.argv[1])
nat_img, nat = load(sys.argv[2])
nat_bounds = native_bounds(nat_img)
print('painel nativo', nat_bounds, 'largura', nat_bounds['right'] - nat_bounds['left'])
scores = []
for name, (ax, x, ay, y, w, h) in REGIONS.items():
    rx, ry = origin(REF, ax, x, ay, y)
    nx, ny = origin(nat_bounds, ax, x, ay, y)
    a = ref[ry:ry + h, rx:rx + w]
    best = (0, 0, 0)
    for dy in range(-3, 4):
        for dx in range(-3, 4):
            b = nat[ny + dy:ny + dy + h, nx + dx:nx + dx + w]
            if b.shape != a.shape:
                continue
            best = max(best, ((np.abs(a - b) <= 16).all(axis=2).mean() * 100, dx, dy))
    scores.append(best[0])
    print(f'{name:<30} {best[0]:5.1f}%  desloc=({best[1]},{best[2]})')
print(f'{"MEDIA":<30} {np.mean(scores):5.1f}%')
