#!/usr/bin/env python3
"""Generate a full favicon pack for GoblinSlop — dark goblin aesthetic."""
from PIL import Image, ImageDraw, ImageFont
import io
import struct

# Colors (dark goblin theme)
BG_COLOR = (15, 20, 15)       # Very dark green-black
EMBLEM_COLOR = (45, 80, 35)   # Goblin green
EYE_YELLOW = (220, 190, 60)   # Amber/amber eyes
SKIN_COLOR = (35, 60, 25)     # Dark goblin skin
MOUTH_COLOR = (180, 40, 40)   # Dark red mouth

def draw_goblin_face(size):
    """Draw a menacing goblin face in the requested size."""
    img = Image.new('RGBA', (size, size), BG_COLOR + (255,))
    draw = ImageDraw.Draw(img)
    
    cx, cy = size // 2, size // 2
    s = size  # for scaling
    
    # Draw rough goblin head shape (oval with sharp angles)
    head_w = int(s * 0.7)
    head_h = int(s * 0.75)
    
    # Head outline - slightly asymmetrical for creepy look
    draw.ellipse(
        [cx - head_w//2, cy - head_h//2, cx + head_w//2, cy + head_h//2],
        fill=SKIN_COLOR
    )
    
    # Ears - pointed and protruding
    ear_size = int(s * 0.2)
    draw.polygon([
        (cx - head_w//2, cy - head_h//3),
        (cx - head_w//2 - ear_size, cy - head_h//2 - ear_size//2),
        (cx - head_w//2 + ear_size//2, cy - head_h//4)
    ], fill=SKIN_COLOR)
    
    draw.polygon([
        (cx + head_w//2, cy - head_h//3),
        (cx + head_w//2 + ear_size, cy - head_h//2 - ear_size//2),
        (cx + head_w//2 - ear_size//2, cy - head_h//4)
    ], fill=SKIN_COLOR)
    
    # Eyes - menacing yellow/gold with vertical slit pupils
    eye_y = int(cy - s * 0.05)
    eye_spacing = int(s * 0.18)
    eye_radius = int(s * 0.06)
    
    for ex in [cx - eye_spacing, cx + eye_spacing]:
        # Eye socket (darker)
        draw.ellipse([ex - eye_radius - 2, eye_y - eye_radius - 2, 
                      ex + eye_radius + 2, eye_y + eye_radius + 2], fill=(20, 35, 15))
        # Yellow iris
        draw.ellipse([ex - eye_radius, eye_y - eye_radius,
                      ex + eye_radius, eye_y + eye_radius], fill=EYE_YELLOW)
        # Slit pupil (vertical ellipse)
        slit_w = int(s * 0.015)
        draw.ellipse([ex - slit_w//2, eye_y - eye_radius*1.3, 
                      ex + slit_w//2, eye_y + eye_radius*1.3], fill=(0, 0, 0))
    
    # Nose - simple bumps
    nose_y = int(cy + s * 0.08)
    draw.ellipse([cx - int(s*0.04), nose_y, cx + int(s*0.04), nose_y + int(s*0.06)], fill=(30, 50, 20))
    
    # Mouth - jagged/grinning with visible teeth
    mouth_y = int(cy + s * 0.2)
    mouth_w = int(s * 0.28)
    # Dark mouth background
    draw.polygon([
        (cx - mouth_w, mouth_y),
        cx, mouth_y + int(s*0.1),
        (cx + mouth_w, mouth_y)
    ], fill=(40, 25, 25))
    
    # White teeth - jagged line
    tooth_count = max(3, size // 8)
    for i in range(tooth_count):
        tx = cx - mouth_w + int((mouth_w * 2 / tooth_count) * i + mouth_w / tooth_count)
        th = int(s * 0.04)
        draw.polygon([
            (tx - s*0.015, mouth_y),
            tx, mouth_y + th,
            (tx + s*0.015, mouth_y)
        ], fill=(220, 230, 210))
    
    # Forehead ridge/brow line for menacing look
    brow_y = int(cy - head_h//3)
    draw.line([(cx - head_w//2 + 5, brow_y), (cx + head_w//2 - 5, brow_y)], 
              fill=(60, 90, 40), width=max(1, size // 30))
    
    return img

def create_ico(image_sizes):
    """Create a multi-frame ICO file from resized PIL images."""
    output = io.BytesIO()
    entries = []
    
    for img in image_sizes:
        buf = img.tobytes('raw', 'RGBA')
        size = len(buf)
        entries.append((img.width, img.height, 0, 0, 1, 0, size, output.tell()))
        output.write(buf)
    
    # ICO header (6 bytes + entries)
    with open('/root/projects/goblinSlop/static/favicon.ico', 'wb') as f:
        f.write(struct.pack('<HHH', 0, 1, len(entries)))  # Reserved, Type=ICO, Count
        for w, h, _, _, _, _, size, offset in entries:
            f.write(struct.pack('<BBBBHHII', 
                              w if w < 256 else 0,  # ICO format uses 0 for 256
                              h if h < 256 else 0,
                              0, 0, 1, 0, size, offset))
        output.seek(0)
        f.write(output.read())

def main():
    static_dir = '/root/projects/goblinSlop/static'
    
    # Generate favicon.ico (multiple sizes in one file)
    sizes_ico = [32, 16]
    favicons = []
    for s in sizes_ico:
        img = draw_goblin_face(s)
        favicons.append(img)
    create_ico(favicons)
    print(f"✅ Created favicon.ico")
    
    # Generate PNG variants
    apple_sizes = [180, 167, 152, 120]
    for s in apple_sizes:
        img = draw_goblin_face(s)
        path = f"{static_dir}/apple-touch-icon-{s}x{s}.png"
        img.save(path, 'PNG')
        print(f"✅ {path}")
    
    # Android Chrome sizes
    android_sizes = [192, 512]
    for s in android_sizes:
        img = draw_goblin_face(s)
        path = f"{static_dir}/android-chrome-{s}x{s}.png"
        img.save(path, 'PNG')
        print(f"✅ {path}")
    
    # Favicon 48px PNG (modern browsers)
    img = draw_goblin_face(48)
    img.save(f"{static_dir}/favicon-48.png", 'PNG')
    print("✅ favicon-48.png")
    
    # Site Web Manifest
    manifest = {
        "name": "GoblinSlop — A Library of Goblin Lore",
        "short_name": "GoblinSlop",
        "description": "A chaotic collection of goblin knowledge, folklore, and dark insights",
        "start_url": "/",
        "display": "standalone",
        "background_color": "#0f140f",
        "theme_color": "#2d5023",
        "icons": [
            {"src": "/static/android-chrome-192x192.png", "sizes": "192x192", "type": "image/png"},
            {"src": "/static/android-chrome-512x512.png", "sizes": "512x512", "type": "image/png"},
            {"src": "/static/favicon.ico", "sizes": "32x32 16x16", "type": "image/x-icon"}
        ]
    }
    
    import json
    with open(f"{static_dir}/site.webmanifest", 'w') as f:
        json.dump(manifest, f, indent=2)
    print("✅ site.webmanifest")

if __name__ == '__main__':
    main()
