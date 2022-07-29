extern crate num_cpus;

fn main() {
  print!("num of cpus: {}", num_cpus::get());
}
