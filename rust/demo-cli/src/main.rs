use std::{
  fs::File,
  io::{BufReader, Read},
};

use clap::{Parser, Subcommand};
// use schema_parser::{parse_schema, trigger_panic};

#[derive(Parser)]
#[clap(name = "demo-cli")]
#[clap(author, version, about, long_about = None)]
struct Cli {
  #[clap(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  #[clap(about = "Print an example struct serialized via serde")]
  Serde,

  #[clap(about = "Print an example struct serialized via tsify")]
  Tsify,

  #[clap(about = "Parse a schema file")]
  Parse(ParseCmd),

  #[clap(about = "Trigger a panic with a given message")]
  Panic(PanicCmd),
}

#[derive(Parser)]
struct ParseCmd {
  #[clap(long)]
  schema: std::path::PathBuf,
}

#[derive(Parser)]
struct PanicCmd {
  #[clap(long, short = 'm')]
  message: String,
}

fn read_schema(schema_path: &std::path::Path) -> Result<String, std::io::Error> {
  let file = File::open(&schema_path)?;
  let mut buf_reader = BufReader::new(file);
  let mut schema_contents = String::new();
  buf_reader.read_to_string(&mut schema_contents)?;
  Ok(schema_contents)
}

fn handle_parse_cmd(cmd: ParseCmd) -> std::io::Result<()> {
  let schema_contents = read_schema(&cmd.schema);
  if let Err(e) = schema_contents {
    eprintln!("Error reading schema file: {}", e);
    std::process::exit(1);
  }

  let schema_contents = schema_contents.unwrap();
  println!("schema_contents:\n{}\n", schema_contents);

  /*
  let schema = parse_schema(schema_contents);

  let schema = schema.unwrap();
  println!("schema:\n{:?}\n", schema);
  */

  Ok(())
}

fn handle_demo_serde_cmd() -> std::io::Result<()> {
  let schema = demo_serde_wasm::example_schema();
  println!("schema:\n{:?}\n", schema);
  Ok(())
}

fn handle_demo_tsify_cmd() -> std::io::Result<()> {
  let schema = demo_tsify_wasm::example_schema();
  println!("schema:\n{:?}\n", schema);
  Ok(())
}

fn handle_panic_cmd(cmd: PanicCmd) -> std::io::Result<()> {
  demo_panic::trigger_panic(cmd.message);
  Ok(())
}

fn main() -> std::io::Result<()> {
  let cmd = Cli::parse();

  match cmd {
    Cli { command: Commands::Serde } => handle_demo_serde_cmd(),
    Cli { command: Commands::Tsify } => handle_demo_tsify_cmd(),
    Cli { command: Commands::Parse(cmd) } => handle_parse_cmd(cmd),
    Cli { command: Commands::Panic(cmd) } => handle_panic_cmd(cmd),
  }
}
