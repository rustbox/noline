use noline::{builder::EditorBuilder, sync_io::std_sync::StdIOWrapper, sync_io::IO};
use std::fmt::Write;
use std::io;
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = io::stdout().into_raw_mode().unwrap();
    let prompt = "> ";

    let mut io = IO::<StdIOWrapper>::new(StdIOWrapper::new());

    let mut editor = EditorBuilder::new_unbounded()
        .with_unbounded_history()
        .build_sync(&mut io)
        .unwrap();

    while let Ok(line) = editor.readline(prompt, &mut io) {
        writeln!(io, "Read: '{}'", line).unwrap();
    }
}
