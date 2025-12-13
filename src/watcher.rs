use crate::filter;
use anyhow::{Context, Result};
use notify::{Event, RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, FileIdMap};
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::mpsc;

/// Setup file watching with debouncing
pub fn setup_watcher(
    paths: Vec<PathBuf>,
) -> Result<(mpsc::Receiver<()>, Debouncer<notify::RecommendedWatcher, FileIdMap>)> {
    let (tx, rx) = mpsc::channel(1);

    let mut debouncer = new_debouncer(
        Duration::from_millis(500),
        None,
        move |result: DebounceEventResult| {
            match result {
                Ok(events) => {
                    for event in events {
                        if should_process_event(&event.event) {
                            if let Some(path) = event.event.paths.first() {
                                if let Some(path_str) = path.to_str() {
                                    println!("{} changed", path_str);
                                }
                            }
                            let _ = tx.blocking_send(());
                            break;
                        }
                    }
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("Watch error: {:?}", error);
                    }
                }
            }
        },
    )
    .context("Failed to create file watcher")?;

    for path in paths {
        let mode = if path.is_dir() {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };

        debouncer
            .watcher()
            .watch(&path, mode)
            .with_context(|| format!("Failed to watch path: {}", path.display()))?;

        if path.is_file() {
            if let Some(parent) = path.parent() {
                debouncer
                    .watcher()
                    .watch(parent, RecursiveMode::NonRecursive)
                    .ok();
            }
        }
    }

    Ok((rx, debouncer))
}

/// Determine if an event should trigger command execution
fn should_process_event(event: &Event) -> bool {
    use notify::EventKind;

    match event.kind {
        EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_) => {
            for path in &event.paths {
                if !filter::should_ignore(path) {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use notify::{event::ModifyKind, EventKind};
    use std::path::PathBuf;

    #[test]
    fn test_should_process_modify_event() {
        let event = Event {
            kind: EventKind::Modify(ModifyKind::Data(notify::event::DataChange::Any)),
            paths: vec![PathBuf::from("test.rs")],
            attrs: Default::default(),
        };

        assert!(should_process_event(&event));
    }

    #[test]
    fn test_should_not_process_ignored_files() {
        let event = Event {
            kind: EventKind::Modify(ModifyKind::Data(notify::event::DataChange::Any)),
            paths: vec![PathBuf::from("test.pyc")],
            attrs: Default::default(),
        };

        assert!(!should_process_event(&event));
    }
}
