mod v_async;
use v_async::run as run_async;

mod v_std;
use v_std::run as run_std;


fn main() {
    run_std();
}