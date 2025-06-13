pub mod helpers;

pub mod template {
    /// Minimal “runner” that prints `Some(n)` or a helpful notice.
    pub fn run<F: FnOnce() -> Option<u64>>(solve: F, n: u32) {
        match solve() {
            Some(ans) => println!("{ans}"),
            None      => eprintln!("Problem {n} not solved yet"),
        }
    }
}

#[macro_export]
macro_rules! solution {
    // zero-argument form builds the function name automatically
    ($n:expr) => {
        ::paste::item! {
            const PROBLEM: u32 = $n;
            fn main() {
                // call `solve_problem_<nnn>()` in the same module
                let answer = [<solve_problem_ $n >]();
                $crate::template::run(|| answer, PROBLEM);
            }
        }
    };
    // fallback: let the caller supply a custom function identifier
    ($n:expr, $func:ident) => {
        const PROBLEM: u32 = $n;
        fn main() {
            $crate::template::run($func, PROBLEM);
        }
    };
}
