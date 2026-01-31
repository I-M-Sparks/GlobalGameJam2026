#!/usr/bin/env python3
"""Generate placeholder card images (Red and Blue letters A-H)"""

from PIL import Image, ImageDraw, ImageFont
import os

# Configuration
CARD_SIZE = 200  # 200x200 pixels per card
LETTERS = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']
PAIRS_DIR = '/home/sparks/Themathar/themathar_game/assets/pairs'

# Colors
RED = (220, 50, 50)
BLUE = (50, 100, 220)
WHITE = (255, 255, 255)
GRAY = (200, 200, 200)

def create_card_image(letter, color, card_type):
    """Generate a card image with a colored letter"""
    # Create image with white background
    img = Image.new('RGB', (CARD_SIZE, CARD_SIZE), WHITE)
    draw = ImageDraw.Draw(img)
    
    # Add border
    border_color = GRAY
    draw.rectangle(
        [(2, 2), (CARD_SIZE-3, CARD_SIZE-3)],
        outline=border_color,
        width=3
    )
    
    # Draw large letter
    try:
        # Try to use a system font, fall back to default
        font_size = 120
        try:
            font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", font_size)
        except:
            font = ImageFont.load_default()
    except:
        font = ImageFont.load_default()
    
    # Draw letter
    bbox = draw.textbbox((0, 0), letter, font=font)
    text_width = bbox[2] - bbox[0]
    text_height = bbox[3] - bbox[1]
    x = (CARD_SIZE - text_width) // 2
    y = (CARD_SIZE - text_height) // 2
    
    draw.text((x, y), letter, fill=color, font=font)
    
    # Add small label at bottom
    label_font_size = 12
    try:
        label_font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", label_font_size)
    except:
        label_font = ImageFont.load_default()
    
    label_color = (50, 50, 50)
    color_name = "Red" if color == RED else "Blue"
    label_text = f"{color_name} {letter}"
    bbox = draw.textbbox((0, 0), label_text, font=label_font)
    label_width = bbox[2] - bbox[0]
    label_x = (CARD_SIZE - label_width) // 2
    draw.text((label_x, CARD_SIZE - 25), label_text, fill=label_color, font=label_font)
    
    return img

def main():
    for letter in LETTERS:
        pair_dir = os.path.join(PAIRS_DIR, letter)
        
        # Create Red (photo) and Blue (art) versions
        red_img = create_card_image(letter, RED, 'photo')
        blue_img = create_card_image(letter, BLUE, 'art')
        
        # Save as photo.png and art.png
        red_img.save(os.path.join(pair_dir, 'photo.png'))
        blue_img.save(os.path.join(pair_dir, 'art.png'))
        
        print(f"✓ Created pair {letter}: photo.png (Red) and art.png (Blue)")

if __name__ == '__main__':
    main()
    print("\n✓ All placeholder assets created!")
