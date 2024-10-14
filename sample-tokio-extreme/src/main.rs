use libc_alloc::LibcAlloc;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Hello, world!");
}
