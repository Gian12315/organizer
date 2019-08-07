use structopt::StructOpt;
use organizer::Config;

fn main() {
    let config = Config::from_args();
    organizer::run(config);
}
