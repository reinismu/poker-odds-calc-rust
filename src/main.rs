use poker_odds_calc::{table::Table, Cards, GameType};
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(
        default_value = "texas_holdem",
        short,
        long,
        parse(try_from_str),
        help = "Select between texas_holdem, shortdeck_holdem and omaha"
    )]
    pub game: GameType,

    #[structopt(
        short,
        long,
        parse(try_from_str),
        help = "Define community cards (ex. `5sTd9cTh`)"
    )]
    pub board: Option<Cards>,

    #[structopt(
        short,
        long = "player",
        parse(try_from_str),
        help = "Define player hand (ex. `AcKh`)"
    )]
    pub players: Vec<Cards>,

    #[structopt(
        default_value = "100000",
        short,
        long,
        help = "Limit number of iterations"
    )]
    pub limit: u64,

    #[structopt(
        short,
        long,
        help = "Run all possible board combinations, regardless limit option"
    )]
    pub exhaustive: bool,

    #[structopt(
        short,
        long,
        help = "ead card(s) to exclude from calculation (ex. `2s2d`)"
    )]
    pub dead: Option<Cards>,

    #[structopt(short, long, help = "Option only available for -g shortdeck_holdem")]
    pub tripsbeatstraight: bool,
}

fn main() {
    let opt: Opt = Opt::from_args();
    println!("{:#?}", opt);
    let table = Table::new(
        opt.players,
        opt.board.unwrap_or(Cards { cards: vec![] }).cards,
        opt.dead.unwrap_or(Cards { cards: vec![] }).cards,
    );
    println!(
        "{:#?}",
        table.get_results(opt.game, opt.limit, opt.tripsbeatstraight, opt.exhaustive)
    );
}
