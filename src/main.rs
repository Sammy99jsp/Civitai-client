#![feature(try_blocks, associated_type_defaults)]
use std::io::stdout;

use app::App;



pub mod app;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut app = App::new()?;
    app.run()?;
    Ok(())
}