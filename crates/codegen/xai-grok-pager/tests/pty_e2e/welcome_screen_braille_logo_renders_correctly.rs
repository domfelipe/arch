// Per-test-case module for the `pty_e2e` integration test crate.
#[allow(unused_imports)]
use super::common::*;

/// 1b. **Welcome screen renders Unicode block-pixel logo correctly.**
///
/// The Arch logo uses multi-byte UTF-8 block geometry (`█ ▄ ▀`, U+2580+).
/// A regression in the writer thread (using `WriteFile` instead of
/// `WriteConsoleW` on Windows, or a missing `SetConsoleOutputCP(65001)`)
/// causes these characters to be misinterpreted as individual legacy
/// code-page bytes, producing garbled output.
///
/// This test asserts that distinctive logo block characters appear intact
/// in the PTY screen buffer.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn welcome_screen_braille_logo_renders_correctly() {
    let content = ContentController::start().await.expect("start content");

    let binary = pager_binary().expect("resolve pager binary");
    // Use a tall terminal so pick_logo() selects the full logo (≥26 rows).
    let mut harness =
        PtyHarness::spawn_with_content(&binary, DEFAULT_ROWS, DEFAULT_COLS, &content, &[])
            .expect("spawn pager");

    harness
        .wait_for_text(WELCOME_SCREEN_SENTINEL, WELCOME_TIMEOUT)
        .expect("welcome text");

    let screen = harness.screen_contents();

    // Multi-byte block / wing geometry from logo07.txt.
    // If the writer mangles UTF-8, these 3-byte sequences fall apart.
    assert!(
        screen.contains('█'),
        "Full block █ (U+2588) not found in screen — \
         logo may be garbled by code-page misinterpretation.\n\
         Screen contents:\n{screen}"
    );
    assert!(
        screen.contains('◥') || screen.contains('◣'),
        "Wing corner blocks ◥/◣ not found in screen — logo may be garbled.\n\
         Screen contents:\n{screen}"
    );

    harness.quit().expect("clean quit");
}
