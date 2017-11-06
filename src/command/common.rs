use clap;

pub trait Command {
  fn handle(&self, args: clap::ArgMatches);
}
