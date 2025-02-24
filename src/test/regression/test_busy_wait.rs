use std::time::{Duration, Instant};

// We've found several real-world examples where the process does a busy wait
// until either some timeout has passed, or work shows up to do. Having Shadow
// eventually move time forward even when the only thing being done is to check
// the time (and maybe make other non-blocking syscalls) prevents these examples
// from deadlocking.
//
// * the golang runtime (https://github.com/shadow/shadow/issues/1968)
// * openblas (https://github.com/shadow/shadow/issues/1792)
// * older versions of Curl (https://github.com/shadow/shadow/issues/1794#issuecomment-985909442)
fn test_wait_for_timeout() {
    let t0 = Instant::now();
    let target = t0 + Duration::from_millis(1);
    while Instant::now() < target {
        // wait
    }
}

fn main() {
    test_wait_for_timeout();
}
