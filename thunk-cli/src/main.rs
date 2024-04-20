use clap::Parser;

fn main() {
    let thunk = thunk_cli::ThunkBuilder::parse().build().unwrap();
    thunk.run();
}
