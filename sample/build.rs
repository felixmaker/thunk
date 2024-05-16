use thunk::{Subsystem, ThunkBuilder, OS};

fn main() {
    if let Ok(thunk) = ThunkBuilder::default()
        .with_os(OS::Windows7)
        .with_subsystem(Subsystem::Windows)
        .build()
    {
        thunk.thunk();
    }
}
