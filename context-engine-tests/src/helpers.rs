//! Helper functions for integration tests.

use std::path::Path;

/// Returns the workspace root path.
/// In our current setup, integration tests run with the workspace root as the
/// working directory.
pub fn workspace_root() -> &'static Path {
    Path::new("..")
}
