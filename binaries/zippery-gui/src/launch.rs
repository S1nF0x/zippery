use std::path::PathBuf;

#[derive(Clone)]
pub enum LaunchMode {
    Browse,
    AddToArchive(Vec<PathBuf>),
    Extract(PathBuf),
}

pub fn parse_args() -> LaunchMode {
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        Some("--add") => {
            let files = args.map(PathBuf::from).collect();
            LaunchMode::AddToArchive(files)
        }
        Some("--extract") => {
            if let Some(path) = args.next() {
                LaunchMode::Extract(PathBuf::from(path))
            } else {
                LaunchMode::Browse
            }
        }

        _ => LaunchMode::Browse
    }
}