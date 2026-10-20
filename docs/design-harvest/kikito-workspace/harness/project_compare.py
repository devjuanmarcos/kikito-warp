"""Dialog parity. usage: modal_compare.py ref.png nat.png [save_dir]
Locates the native dialog by best whole-box alignment near the expected spot, then scores
regions (relative to the dialog origin) raw (tol 10) and blurred (sigma 1, tol 16, shift +-2)."""
import sys
import numpy as np
from PIL import Image, ImageFilter

REF_BOX = (430, 241, 420, 318)
NAT_GUESS = (430, 241)
REGIONS = {
    'header': (0, 0, 420, 60),
    'descricao': (0, 60, 420, 36),
    'nome': (0, 96, 420, 72),
    'diretorio': (0, 168, 420, 90),
    'rodape': (0, 258, 420, 60),
    'TOTAL': (0, 0, 420, 318),
}


def load(path, blur=False):
    im = Image.open(path).convert('RGB')
    if blur:
        im = im.filter(ImageFilter.GaussianBlur(1))
    return np.asarray(im).astype(int)


def score(a, b, tol):
    return (np.abs(a - b) <= tol).all(axis=2).mean() * 100


ref, nat = load(sys.argv[1]), load(sys.argv[2])
refb, natb = load(sys.argv[1], True), load(sys.argv[2], True)
rx, ry, w, h = REF_BOX
best = None
for dy in range(-30, 31):
    for dx in range(-6, 7):
        nx, ny = NAT_GUESS[0] + dx, NAT_GUESS[1] + dy
        s = score(refb[ry:ry + h, rx:rx + w], natb[ny:ny + h, nx:nx + w], 16)
        if best is None or s > best[0]:
            best = (s, nx, ny)
_, nx, ny = best
print(f'origem nativa = ({nx},{ny})  esperado ({rx},{ry})  desloc=({nx - rx},{ny - ry})')
for name, (x, y, rw, rh) in REGIONS.items():
    a = ref[ry + y:ry + y + rh, rx + x:rx + x + rw]
    b = nat[ny + y:ny + y + rh, nx + x:nx + x + rw]
    raw = score(a, b, 10)
    blurred = 0
    for sy in range(-2, 3):
        for sx in range(-2, 3):
            ab = refb[ry + y:ry + y + rh, rx + x:rx + x + rw]
            bb = natb[ny + y + sy:ny + y + sy + rh, nx + x + sx:nx + x + sx + rw]
            blurred = max(blurred, score(ab, bb, 16))
    print(f'{name:<10} bruto {raw:5.1f}%   desfoque {blurred:5.1f}%')
    if len(sys.argv) > 3:
        side = np.concatenate([a, b, (np.abs(a - b) * 4).clip(0, 255)], axis=1).astype('uint8')
        Image.fromarray(side).resize((side.shape[1] * 2, side.shape[0] * 2), Image.NEAREST).save(
            f'{sys.argv[3]}/cmp_{name}.png')
