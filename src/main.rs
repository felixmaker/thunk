use clap::Parser;

fn main() {
    let thunk = thunk::ThunkBuilder::parse().build().unwrap();
    thunk.run();
}
