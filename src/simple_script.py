#!/usr/bin/env python3

"""
Author: John George
Fun Fact: I sleep on a full-wave 1960s waterbed
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
    print("""
My professional path began when I decided to step away from college and join
the military while determining what career I wanted to pursue. I remember
speaking with a recruiter who recommended working on aircraft. Sitting in
his Tennessee office on a beautiful spring day, he asked, “Who wouldn’t want
to be outside in this weather, getting paid to work on the most cutting-edge
aircraft ever made?” I looked more closely at where those aircraft bases were
located and decided that working outdoors might be pleasant in Tennessee but
considerably less appealing in Anchorage, Alaska. Instead, I asked for a job
that would allow me to work from a chair in a temperature-controlled room. He
suggested cyber.

Although I had never considered information technology as a profession, I fell
in love with the field after completing technical training and beginning
full-time work on base. I have always considered myself a learn-it-all rather
than a know-it-all, and I had found an ever-changing field that rewarded
constant learning. I worked exclusively in the military for approximately eight
years before joining Microsoft as an engineer in 2019. Despite becoming an
engineering expert in my field, I remained driven to explore less-developed
areas of technology. That desire eventually led me to Oak Ridge National
Laboratory in 2025, where I now work at the cutting edge of artificial
intelligence and emerging technologies.
""")

def get_family():
    # find file reletive to script
    image_path = Path(__file__).resolve().parent.parent / "data" / "family.png"
    # Load from a local file 
    my_art = AsciiArt.from_image(str(image_path))
    my_art.to_terminal(columns=80)
    print("""
I met my wife in 2014 when we both worked at Chick-Fil-A in Alcoa, TN.  I was
working as a fry cook, and she was the newest cashier in training.  After
coming back from basic training, we reconnected and decided to see where we
would go.  I knew it would be marriage, but she had to catch up. We got married
in 2017 and have grown our family with two daughters.  My oldest, Athena, is
six years old who likes to play sports and be outside. My youngest, Ahsoka, is
a two-year-old girly girl who cannot be bothered to even let grass touch her
barefoot. I am a very proud father, and even more proud as a husband.  Before I
got accepted to this class, my wife learned that she would also be leaving her
role as a stay-at-home mom of six years.  She is now full time at UTK online
becoming a master’s in social work.  We also have opened our home to three
dogs.  We have a highly energetic Irish Setter named Ayla, I am the emotional
support human for our Irish doodle named Arya, and we have an old yorkie that
weighs three pounds and runs the house named Ava.
""")

def get_hobbies():
    # find file reletive to script
    image_path = Path(__file__).resolve().parent.parent / "data" / "beer.png"
    # Load from a local file 
    my_art = AsciiArt.from_image(str(image_path))
    my_art.to_terminal(columns=80)
    print("""
If I ever manage to carve out free time between family, work, and school, I
enjoy homebrewing beers, wines, and meads.  I am currently buying pre-bundled beer
ingredient kits, but I hope to be able to craft my own recipes one day.  I have
two muscadine bushes that I inherited from the previous house owner and now use
it for homemade muscadine wine.  Thanks to my wife I am an avid traveler,
focusing right now on the United States, and cruises.  She plans at least one
cruise a year and ends up driving around this side of the country for two other
vacations each year.  I am also a learn-it-all that loves to challenge myself
on new projects and help my friends with their passion projects.  After having
mastered Python and VS Code, I switched my daily editor to Vim. That is why the
navigation in this script is h, j, k, and l, which correspond to left, down,
up, and right in Vim. I have also rewritten this app in Rust on a separate
branch, because I truly enjoy pushing into new ideas and trying other ways to
keep myself challenged and informed.
""")

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
                print("Invalid input.")

        user_input = get_user_input()
