// Integration test for SigmaOS-installation-logging
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use installation_logging as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
