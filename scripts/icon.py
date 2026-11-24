#!/usr/bin/env python3
"""Draws the app icon and builds the .icns macOS wants.

Drawn rather than shipped as a binary: a terminal that renders its own
rounded corners can draw its own rounded square, and a script can be read.
"""
import math
import struct
import subprocess
import sys
import zlib
from pathlib import Path

# The same values assets/caret.svg uses, as fractions of the icon's side.
# That file is the drawing; this is the rasteriser, and neither is allowed to
# drift from the other.
BACKDROP = (0x15, 0x19, 0x1F)
EDGE = (0x2E, 0x37, 0x44)
PROMPT = (0x7E, 0xE7, 0x87)
CURSOR = (0x6C, 0xA4, 0xF8)

CORNER = 0.225
INSET = 0.06
HAIRLINE = 0.016

PROMPT_X = 0.26530
PROMPT_TIP = 0.48970
PROMPT_TOP = 0.385
PROMPT_BOTTOM = 0.615
PROMPT_WEIGHT = 0.075

CURSOR_X = 0.62720
CURSOR_WIDTH = 0.145
CURSOR_RADIUS = 0.03516


def rounded(px, py, size, radius):
    """Coverage of a rounded square, antialiased at its edge."""
    half = size / 2
    dx = abs(px - half) - (half - radius)
    dy = abs(py - half) - (half - radius)
    # The standard rounded-rectangle distance: the corner is a circle, the
    # sides are straight, and the two cases meet without a seam.
    outside = math.hypot(max(dx, 0.0), max(dy, 0.0))
    inside = min(max(dx, dy), 0.0)
    distance = outside + inside - radius
    return min(max(0.5 - distance, 0.0), 1.0)


def bar(px, py, x0, y0, x1, y1, thickness):
    """Coverage of a thick line segment, with rounded ends."""
    vx, vy = x1 - x0, y1 - y0
    length = math.hypot(vx, vy)
    if length == 0:
        return min(max(0.5 + thickness / 2 - math.hypot(px - x0, py - y0), 0.0), 1.0)
    t = max(0.0, min(1.0, ((px - x0) * vx + (py - y0) * vy) / (length * length)))
    near = math.hypot(px - (x0 + vx * t), py - (y0 + vy * t))
    return min(max(0.5 + thickness / 2 - near, 0.0), 1.0)


def box(px, py, x, y, w, h, radius):
    """Coverage of a rounded rectangle at an arbitrary place."""
    dx = abs(px - (x + w / 2)) - (w / 2 - radius)
    dy = abs(py - (y + h / 2)) - (h / 2 - radius)
    outside = math.hypot(max(dx, 0.0), max(dy, 0.0))
    inside = min(max(dx, dy), 0.0)
    return min(max(0.5 - (outside + inside - radius), 0.0), 1.0)


def blend(under, over, alpha):
    return tuple(round(o * alpha + u * (1 - alpha)) for u, o in zip(under, over))


def draw(size):
    """The icon at `size`, as RGBA rows.

    A shell prompt and the block cursor waiting after it: what the thing is,
    rather than a letter standing for its name.
    """
    s = size
    inset = s * INSET
    span = s - inset * 2
    radius = s * CORNER
    # Floors in whole pixels, not fractions of them. A 16 pixel icon drawn
    # to the same fractions as a 512 one comes out as mush: the hairline
    # lands between pixels and the gap closes up.
    hair = max(round(s * HAIRLINE), 1.0)

    weight = max(s * PROMPT_WEIGHT, 2.0)
    top, bottom = s * PROMPT_TOP, s * PROMPT_BOTTOM
    left, tip = s * PROMPT_X, s * PROMPT_TIP
    middle = s * 0.5

    cursor_x = s * CURSOR_X
    cursor_w = max(s * CURSOR_WIDTH, 3.0)
    cursor_r = max(s * CURSOR_RADIUS, 1.0)

    rows = []
    for y in range(s):
        row = bytearray()
        for x in range(s):
            px, py = x + 0.5, y + 0.5
            cover = rounded(px - inset, py - inset, span, radius)
            if cover <= 0:
                row += bytes((0, 0, 0, 0))
                continue

            # A hairline of the lighter edge, so the shape reads on a dark
            # desktop instead of dissolving into it.
            fill = rounded(px - inset - hair, py - inset - hair, span - hair * 2, radius)
            colour = blend(EDGE, BACKDROP, fill)

            chevron = max(
                bar(px, py, left, top, tip, middle, weight),
                bar(px, py, left, bottom, tip, middle, weight),
            )
            colour = blend(colour, PROMPT, chevron)

            cursor = box(px, py, cursor_x, top, cursor_w, bottom - top, cursor_r)
            colour = blend(colour, CURSOR, cursor)

            row += bytes((*colour, round(cover * 255)))
        rows.append(bytes(row))
    return rows


def png(rows, size):
    raw = b"".join(b"\x00" + row for row in rows)

    def chunk(kind, body):
        head = struct.pack(">I", len(body)) + kind
        return head + body + struct.pack(">I", zlib.crc32(kind + body) & 0xFFFFFFFF)

    return (
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", struct.pack(">IIBBBBB", size, size, 8, 6, 0, 0, 0))
        + chunk(b"IDAT", zlib.compress(raw, 9))
        + chunk(b"IEND", b"")
    )


def main(out: Path):
    iconset = out.with_suffix(".iconset")
    iconset.mkdir(parents=True, exist_ok=True)
    # The sizes macOS asks for, each at one and two times.
    for base in (16, 32, 128, 256, 512):
        for scale in (1, 2):
            size = base * scale
            suffix = "" if scale == 1 else "@2x"
            name = f"icon_{base}x{base}{suffix}.png"
            (iconset / name).write_bytes(png(draw(size), size))
    subprocess.run(["iconutil", "-c", "icns", str(iconset), "-o", str(out)], check=True)
    print(f"wrote {out}")


if __name__ == "__main__":
    main(Path(sys.argv[1] if len(sys.argv) > 1 else "macos/caret.icns"))
