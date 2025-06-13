A rust project template for solving project euler problems.

Use commands to help manage the problems:
- `cargo scaffold <n>` - generates a file for solving problem <n>, complete with url link, skeletal function, and test
- `cargo solve <n>` - runs the solution for the given problem <n>
- `cargo all` - runs all solved solutions
- `cargo time <n> <iters>` - times the solution to problem <n>, with an optional <iters> parameter (defaulting to 100) to specify how many runs

You can add any helper functions in `./project_euler/src/helpers/mod.rs`

**My Problem 32 solution has been included as an example of how the template works. If you do not care for spoilers, feel free to delete `./project_euler/src/bin/problem_032.rs`.**