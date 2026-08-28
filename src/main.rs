/*
Author: John George
Fun Fact: I sleep on a full-wave 1960s waterbed
*/
use std::{
    io::{self, Write},
    path::Path,
};
use viuer::{Config, print_from_file};

fn get_user_input() -> io::Result<String> {
    let mut input = String::new();
    print!(
        "Use the following buttons to learn more!\n\
         H: Work J: Family K: Hobbies L: Quit\n\
         Input: "
    );

    io::stdout().flush()?; // Ensure the prompt is displayed before reading input
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_uppercase())
}

fn show_image(filename: &str) {
    let path = Path::new("data").join(filename);

    let config = Config {
        height: Some(12),
        absolute_offset: false,
        ..Default::default()
    };

    if let Err(error) = print_from_file(&path, &config) {
        eprintln!("Failed to display image: {}", error);
    }

    println!();
}

fn get_work() {
    show_image("usaf.jpg");
    println!(
        r#"My professional path began when I decided to step away from college and join
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
"#
    );
}

fn get_family() {
    show_image("family.png");
    println!(
        r#"I met my wife in 2014 when we both worked at Chick-Fil-A in Alcoa, TN.  I was
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
"#
    );
}
fn get_hobbies() {
    show_image("beer.png");
    println!(
        r#"If I ever manage to carve out free time between family, work, and school, I
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
"#
    );
}

fn main() -> io::Result<()> {
    io::stdout().flush()?;

    loop {
        let user_input = get_user_input()?;

        // ASCII terminal magic to clear the screen and move the cursor to the top-left corner
        print!("\x1B[2J\x1B[H");
        io::stdout().flush()?;

        match user_input.as_str() {
            "H" => get_work(),
            "J" => get_family(),
            "K" => get_hobbies(),
            "L" => break,
            _ => println!("Invalid input."),
        }
    }

    Ok(())
}
