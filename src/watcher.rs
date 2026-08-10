use crate::builder;
use crate::config::Config;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::time::Duration;

/// How long the event stream must stay quiet before rebuilding, so that a single
/// editor save triggers a single rebuild
const DEBOUNCE: Duration = Duration::from_millis(200);

/// Watch the source directories and the config file, rebuilding on every change.
/// Only returns if the event stream ends or a watcher cannot be created.
pub fn watch(config_path: &Path, mut config: Config) -> Result<(), io::Error> {
    let config_path = absolute(config_path);

    loop {
        let paths = WatchPaths::new(&config_path, &config);
        let (tx, rx) = channel();
        // Bound to a variable: dropping the watcher stops the event stream
        let _watcher = create_watcher(tx, &paths)?;

        for path in paths.watched() {
            println!("Watching {}", path.display());
        }

        match run(&rx, &paths, &config_path, &mut config) {
            Outcome::Stop => return Ok(()),
            // The config now points elsewhere, restart with a fresh watcher
            Outcome::Rewatch => continue,
        }
    }
}

enum Outcome {
    Stop,
    Rewatch,
}

fn run(
    rx: &Receiver<Result<Event, notify::Error>>,
    paths: &WatchPaths,
    config_path: &Path,
    config: &mut Config,
) -> Outcome {
    loop {
        let changed = match collect_changed_paths(rx) {
            Some(changed) => changed,
            None => return Outcome::Stop,
        };

        let changed: Vec<PathBuf> = changed
            .into_iter()
            .filter(|path| paths.is_source(path))
            .collect();
        if changed.is_empty() {
            continue;
        }

        // Reload the config when it changed, keeping the previous one on error
        if changed.iter().any(|path| path == config_path) {
            println!("\nConfig changed, reloading {}", config_path.display());
            match Config::load_from_file(config_path) {
                Ok(new_config) => {
                    let rewatch = new_config.input_directory != config.input_directory
                        || new_config.template_directory != config.template_directory;
                    *config = new_config;
                    build(config);
                    if rewatch {
                        return Outcome::Rewatch;
                    }
                    continue;
                }
                Err(e) => {
                    eprintln!(
                        "Error loading config file '{}': {}",
                        config_path.display(),
                        e
                    );
                    continue;
                }
            }
        }

        println!("\nRebuilding...");
        build(config);
    }
}

fn build(config: &Config) {
    if let Err(e) = builder::build_site(config) {
        eprintln!("Error {e}");
    }
}

/// Absolute paths of everything the watcher cares about. notify reports events using
/// the paths it was given, so they are made absolute up front to be comparable.
struct WatchPaths {
    config: PathBuf,
    config_directory: PathBuf,
    input: PathBuf,
    template: PathBuf,
    output: PathBuf,
}

impl WatchPaths {
    fn new(config_path: &Path, config: &Config) -> Self {
        let config_file = absolute(config_path);
        let config_directory = config_file
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| absolute(Path::new(".")));

        Self {
            config: config_file,
            config_directory,
            input: absolute(&config.input_directory),
            template: absolute(Path::new(&config.template_directory)),
            output: absolute(&config.output_directory),
        }
    }

    fn watched(&self) -> [&Path; 3] {
        [&self.input, &self.template, &self.config]
    }

    /// A change is worth a rebuild when it touches an article, a template or the
    /// config file, and does not come from the site we just generated
    fn is_source(&self, path: &Path) -> bool {
        if path.starts_with(&self.output) {
            return false;
        }
        path == self.config || path.starts_with(&self.input) || path.starts_with(&self.template)
    }
}

/// Watch the input and template directories recursively, plus the directory holding
/// the config file (editors replace files on save, so watching the file itself would
/// miss changes)
fn create_watcher(
    tx: std::sync::mpsc::Sender<Result<Event, notify::Error>>,
    paths: &WatchPaths,
) -> Result<RecommendedWatcher, io::Error> {
    let mut watcher = notify::recommended_watcher(move |res| {
        // The receiver outlives the watcher, so a send failure only means shutdown
        let _ = tx.send(res);
    })
    .map_err(io::Error::other)?;

    for path in [&paths.input, &paths.template] {
        watcher
            .watch(path, RecursiveMode::Recursive)
            .map_err(io::Error::other)?;
    }
    watcher
        .watch(&paths.config_directory, RecursiveMode::NonRecursive)
        .map_err(io::Error::other)?;

    Ok(watcher)
}

/// Block until something changes, then drain the event stream until it goes quiet.
/// Returns None once the watcher is gone.
fn collect_changed_paths(rx: &Receiver<Result<Event, notify::Error>>) -> Option<Vec<PathBuf>> {
    let mut paths = Vec::new();
    push_paths(&mut paths, rx.recv().ok()?);

    loop {
        match rx.recv_timeout(DEBOUNCE) {
            Ok(res) => push_paths(&mut paths, res),
            Err(RecvTimeoutError::Timeout) | Err(RecvTimeoutError::Disconnected) => {
                return Some(paths);
            }
        }
    }
}

fn push_paths(paths: &mut Vec<PathBuf>, res: Result<Event, notify::Error>) {
    match res {
        // Reading a file is an event too: generating the site would otherwise
        // trigger an endless stream of rebuilds
        Ok(event) if event.kind.is_access() => {}
        Ok(event) => paths.extend(event.paths),
        Err(e) => eprintln!("Warning: watch error: {e}"),
    }
}

/// Make a path absolute and free of `.` / `..` components without requiring it to
/// exist, so prefix comparisons against event paths hold
fn absolute(path: &Path) -> PathBuf {
    std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf())
}
