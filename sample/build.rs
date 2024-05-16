use thunk::{ThunkBuilder, OS};

fn main() {
    if let Ok(thunk) = ThunkBuilder::default().with_os(OS::WindowsXP).build() {
        thunk.thunk();
    }
}
