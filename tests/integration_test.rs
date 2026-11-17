// Integration test for SigmaOS-p11-kit
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_p11_kit as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
