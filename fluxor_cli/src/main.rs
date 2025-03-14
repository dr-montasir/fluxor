use fluxor_cli::{Args, Commands, create_fluxor_web_project};
use fluxor_cli::Parser;

fn main () {
   let args = Args::parse();

   match args.command {
      Commands::New { name, version, example } => {
         create_fluxor_web_project(&name, &version, &example);
      }
   }
}