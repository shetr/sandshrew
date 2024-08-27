use {
    std::{
        env,
        io,
    },
    winresource::WindowsResource,
};

// got from https://stackoverflow.com/questions/30291757/attaching-an-icon-resource-to-a-rust-application
fn main() -> io::Result<()> {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            // This path can be absolute, or relative to your crate root.
            .set_icon("assets/favicon.ico")
            .compile()?;
    }
    Ok(())
}