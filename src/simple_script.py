#!/usr/bin/env python3

"""
File Name: simple_script.py
Author: John George
Date Created: August 26, 2026
Version: 1.0
Description: A short script for DSE511.
Fun Fact: I sleep on a waterbed
"""

from ascii_magic import AsciiArt
from pathlib import Path

if __name__ == "__main__":
    # find file reletive to script
    image_path = Path(__file__).resolve().parent.parent / "data" / "usaf.jpg"
    # Load from a local file 
    my_art = AsciiArt.from_image(str(image_path))
    my_art.to_terminal(columns=80)
