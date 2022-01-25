

fn get_ntg_args(n: usize) -> String {
  std::env::args().nth(n).unwrap()
}

#[derive(Debug)]
pub struct Args {
  pub image_1: String,
  pub image_2: String,
  pub output: String
}

impl Args {
  pub fn new() -> Self {
    Args {
      image_1: get_ntg_args(1),
      image_2: get_ntg_args(2),
      output: get_ntg_args(3)
    }
  }
}