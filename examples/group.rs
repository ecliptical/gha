use gha::{error, group, notice};

fn main() {
    group("Test 1", || notice!(title = "Test", "This is a test!"));
    if let Err(e) = group("Test 2", || {
        notice!(title = "Test 2", "Maybe error?");
        Err::<(), _>("There was an error!")
    }) {
        error!(title = "Test 2", "Error: {e}");
    }
}
