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
        r#"My academic career has followed a non-traditional trajectory. It was assumed
that after graduating high school, I would follow in my father and grandfather's
footsteps. Both of them attended UTK and became dentists. My uncle also attended
UTK and went on to become an orthopedic surgeon. I presume there was some
brotherly competition involved. Unfortunately, I did not care to put my hands in
:wqpeople's mouths all day. Not wanting to waste money and time in college, I
decided I would join the military until I had decided on a career path. I
remember talking with a recruiter who recommended a job working on aircraft.
While sitting in his office in Tennessee one spring, he said something like,
"Who wouldn't want to be outside in this beautiful weather, getting paid to work
on the most cutting-edge aircraft ever made?" That was true, but I also knew the
Air Force had military bases in San Antonio, Texas, in the summer and Anchorage,
Alaska, in the winter. I told him that sounded nice here today, but not in those
other places, so I would prefer a position that was entirely indoors. He said I
would be a good fit for "cyber." I had never considered information technology
professionally, but I truly fell in love with the work. I have always been a
learn-it-all, not a know-it-all, and there is always more to discover in the
field. While in the military, I earned my BS in information technology security.
I worked there for about eight years before I was hired as an engineer at
Microsoft in 2019. I continued my education while working at Microsoft and went
back to school for my MS in data analytics shortly after my first daughter was
born. Despite being the engineering expert in my field at Microsoft, I still
felt the drive to further my education and explore the unknown, which led me to
Oak Ridge National Laboratory in 2025. I am now furthering my academic career
with a PhD program to drive the cutting edge even further. From DSE511, I hope
to deepen my understanding of data science research methods, strengthen my
ability to analyze and communicate findings, and apply those skills to
meaningful research problems in AI and national laboratory work.
"#
    );
}

fn get_family() {
    show_image("family.png");
    println!(
        r#"I met my wife in 2015. We both worked at Chick-fil-A in Alcoa. I was the
fry cook, in charge of cooking chicken, and she was the cashier, in charge of
selling it. It was a match made in heaven. We were married in 2017 and have two
daughters. My oldest, Athena, is six years old. She is a rough-and-tumble tomboy
who likes wrestling and playing outside. My youngest, Ahsoka, is a two-year-old
girly girl, a princess who will not get in the grass barefoot. I am a very
proud father and husband. I am particularly proud of my wife, who recently left
her role as a stay-at-home mom of six years to return to UTK to pursue her
master's in social work. I also have three dogs: a highly energetic Irish setter
named Ayla, a couch-potato Irish doodle named Arya, and a 10-year-old Yorkie
named Ava who weighs a whopping three pounds.
"#
    );
}
fn get_hobbies() {
    show_image("beer.png");
    println!(
        r#"If I ever manage to carve out free time between family, work, and school, I
enjoy homebrewing beers, wines, and meads. I currently buy pre-bundled beer
ingredient kits, but I hope to be able to craft my own recipes one day. I have
two muscadine bushes that I use for homemade muscadine wine. I am also an avid
cruiser. I make sure I go on at least one cruise a year. As I have mentioned
before, I am also a learn-it-all that loves to challenge myself. After having
mastered Python and VS Code, I switched my daily editor to Vim. That is why the
navigation in this script is h, j, k, and l, which correspond to left, down, up,
and right in Vim. I have also rewritten this app in Rust on a separate branch.
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
