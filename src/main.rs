mod v_async;
use v_async::run as run_async;

mod v_std;
use v_std::run as run_std;

const addr: &str = "0.0.0.0:8080";

fn main() {
    run_std();
}