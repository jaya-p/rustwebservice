// build: cd <project-root-directory> && cargo build
// run: cd <project-root-directory> && cargo run --bin main
// run (other): cd <project-root-directory> && cargo run

fn main() {
  println!("{}", rustwebservice::helloworld::helloworld());
}