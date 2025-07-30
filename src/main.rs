use fedit::editor::Editor;

use clap::Parser;
use std::io;

#[derive(Parser)]
#[command(name = "fedit")]
#[command(about = "A simple text editor")]
struct Cli {
    file: Option<String>,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let mut editor = Editor::new();

    if let Some(filename) = cli.file {
        editor.load_file(&filename)?;
    }

    editor.run()
}
