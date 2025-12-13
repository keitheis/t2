use crate::filter::IgnoreFilter;
use crate::ui;
use anyhow::{Context, Result};
use notify::{Event, RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, FileIdMap};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

/// Setup file watching with debouncing
///
/// Returns a channel receiver that receives notifications when files change
/// Uses notify crate for event-based watching (improvement over Python's polling)
pub fn setup_watcher(
    paths: Vec<PathBuf>,
    filter: Arc<IgnoreFilter>,
) -> Result<(mpsc::Receiver<()>, Debouncer<notify::RecommendedWatcher, FileIdMap>)> {
    let (tx, rx) = mpsc::channel(100);

    // Create debouncer with 500ms delay
    let mut debouncer = new_debouncer(
        Duration::from_millis(500),
        None,
        move |result: DebounceEventResult| {
            match result {
                Ok(events) => {
                    for event in events {
                        if should_process_event(&event.event, &filter) {
                            // Print which file changed (like Python version)
                            if let Some(path) = event.event.paths.first() {
                                ui::print_file_changed(path);
                            }
                            // Send notification to trigger command execution
                            let _ = tx.blocking_send(());
                            break; // Only send one notification per batch
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

    // Add paths to watcher
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

        // Also watch parent directory for individual files
        if path.is_file() {
            if let Some(parent) = path.parent() {
                debouncer
                    .watcher()
                    .watch(parent, RecursiveMode::NonRecursive)
                    .ok(); // Ignore errors for parent watching
            }
        }
    }

    Ok((rx, debouncer))
}

/// Determine if an event should trigger command execution
fn should_process_event(event: &Event, filter: &IgnoreFilter) -> bool {
    use notify::EventKind;

    // Only process modify, create, and remove events
    match event.kind {
        EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_) => {
            // Check if any of the paths should not be ignored
            for path in &event.paths {
                if !filter.should_ignore(path) {
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
        let filter = Arc::new(IgnoreFilter::new());
        let event = Event {
            kind: EventKind::Modify(ModifyKind::Data(notify::event::DataChange::Any)),
            paths: vec![PathBuf::from("test.rs")],
            attrs: Default::default(),
        };

        assert!(should_process_event(&event, &filter));
    }

    #[test]
    fn test_should_not_process_ignored_files() {
        let filter = Arc::new(IgnoreFilter::new());
        let event = Event {
            kind: EventKind::Modify(ModifyKind::Data(notify::event::DataChange::Any)),
            paths: vec![PathBuf::from("test.pyc")],
            attrs: Default::default(),
        };

        assert!(!should_process_event(&event, &filter));
    }

    #[test]
    fn test_should_process_create_event() {
        let filter = Arc::new(IgnoreFilter::new());
        let event = Event {
            kind: EventKind::Create(notify::event::CreateKind::File),
            paths: vec![PathBuf::from("new_file.txt")],
            attrs: Default::default(),
        };

        assert!(should_process_event(&event, &filter));
    }
}
