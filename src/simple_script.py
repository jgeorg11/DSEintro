#!/usr/bin/env python3

"""
File Name: simple_script.py
Author: John George
Date Created: August 26, 2026
Version: 1.0
Description: A short script for DSE511.
Fun Fact: I sleep on a waterbed
"""

import os
from ascii_magic import AsciiArt
from pathlib import Path

# Function to clear the screen based on the operating system
def clear_screen():
    # 'nt' means Windows, otherwise it is Linux/Mac
    os.system('cls' if os.name == 'nt' else 'clear')

def get_user_input():
    user_input = input("""Use the following buttons to learn more!
H: Work J: Family K: Hobbies L: Quit
Input: """)
        
    clear_screen()
    return user_input.upper()

def get_work():
    # find file reletive to script
    image_path = Path(__file__).resolve().parent.parent / "data" / "usaf.jpg"
    # Load from a local file 
    my_art = AsciiArt.from_image(str(image_path))
    my_art.to_terminal(columns=80)
    print("\nI started my professional career in 2012 in the United States Air Force,\nwhere I currently serve as a traditional guardsman as a Senior Watch Officer\n")
    print("I worked for Microsoft as an engineer for 6 years before recently moving to Oak\nRidge National Lab as a Senior AI Architect\n")

def get_family():
    # find file reletive to script
    image_path = Path(__file__).resolve().parent.parent / "data" / "family.png"
    # Load from a local file 
    my_art = AsciiArt.from_image(str(image_path))
    my_art.to_terminal(columns=80)
    print("\nI have been married for 9 years to my wife, Elizabeth. And have two beautiful\ndaughters Ahsoka and Athena.\n")
    print("My wife is also a UTK student. Getting her Master's in social work.\n\nI also have three dogs: Ayla, Arya, and Ava.\n")

def get_hobbies():
    # find file reletive to script
    image_path = Path(__file__).resolve().parent.parent / "data" / "beer.png"
    # Load from a local file 
    my_art = AsciiArt.from_image(str(image_path))
    my_art.to_terminal(columns=80)
    print("\nI enjoy homebrewing beers, wines, and meads. I also enjoy bourbon and cigars.\n")

def quit():
    pass

if __name__ == "__main__":
    clear_screen()

    user_input = get_user_input()

    while user_input != "L":
        match user_input:
            case "H":
                get_work()
            case "J":
                get_family()
            case "K":
                get_hobbies()
            case _:
                print("invalid input")

        user_input = get_user_input()
