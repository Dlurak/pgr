mod generate;

use clap::{Parser, ArgAction};

#[derive(Parser, Debug)]
#[clap(
    name = "pgr",
    about = "Password Generator Rust is a cli to generate secure passwords locally and blazingly fast",
    after_help = "For more information and the source code, visit https://github.com/dlurak/pgr.",
    setting(clap::AppSettings::ColoredHelp)
)]
struct Args {
    #[clap(
        short,
        long,
        default_value_t = 16,
        help = "The length of the password",
        aliases = &["len", "leng"]
    )]
    length: usize,
    #[clap(
        short = 'L',
        long,
        action = ArgAction::SetTrue,
        help = "Should lowercase letters be excluded",
        aliases = &["lower"]
    )]
    lowercase: bool,
    #[clap(
        short,
        long,
        action = ArgAction::SetTrue,
        help = "Should uppercase letteres be excluded",
        aliases = &["upper"]
    )]
    uppercase: bool,
    #[clap(short, long, action = ArgAction::SetTrue, help = "Should digits be excluded")]
    digit: bool,
    #[clap(
        short,
        long,
        action = ArgAction::SetTrue,
        help = "Should special charachters be excluded",
        aliases = &["spec"]
    )]
    special: bool,
}

fn main() {
    let cli = Args::parse();

    let char_sets = vec![
        (cli.lowercase, generate::CharSets::Lower),
        (cli.uppercase, generate::CharSets::Upper),
        (cli.digit, generate::CharSets::Digit),
        (cli.special, generate::CharSets::Special),
    ];
    let char_sets = char_sets.iter().filter_map(|(flag, charset)| {
        if *flag {
            None
        } else {
            Some(charset.get_string())
        }
    });
    let char_sets = char_sets.collect::<Vec<_>>();
    let char_pool = char_sets.join("");

    let pwd = generate::create_string(&cli.length, || {
        generate::random_char(&char_pool).unwrap_or('%')
    });
    match pwd {
        Ok(s) => println!("{}", s),
        Err(_) => eprintln!("Something went wrong :("),
    }
}
