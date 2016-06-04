use liquid;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*; // read_to_string
use liquid::{Renderable, Context, Value};

use app::App;
//use Parser;

// TODO: should probably make this renderable in a nicer way
//impl Renderable for App {
//    fn render(&self, _context: &mut Context) -> Result<Option<String>, Error>{
//        Ok(Some(self.text.to_uppercase()))
//    }
//}


pub fn gencomp(app: &App) -> String {
  // TODO: sane location for this template
  let comp_pth = Path::new(".").join("completion").join("bash.sh");
  let mut f = File::open(&comp_pth).unwrap();
  let mut data = String::new();
  f.read_to_string(&mut data).unwrap();

  let tmpl = liquid::parse(&data, Default::default()).unwrap();

  let mut context = Context::new();
  context.set_val("binname", Value::Str("foo".into()));

  // Completable: global flags
  let mut flags = vec![];
  for f in &app.p.short_list {
      flags.push(format!("-{}", f));
  }
  for f in &app.p.long_list {
    flags.push(format!("--{}", f));
  }
  context.set_val("flags_spaces", Value::Str(flags.join(" ")));

  // Completable: subcommands in global scope
  let mut subcmds = vec![];
  for sub in &app.p.subcommands {
      subcmds.push(format!("{}", sub));
  }
  context.set_val("subcommands", Value::Str(subcmds.join(" ")));
  context.set_val("subcommands_pattern", Value::Str(subcmds.join("|")));

  // TODO: flags for subcommands
  // TODO: args for subcommands

  // TODO: recurse

  let output = tmpl.render(&mut context);
  output.unwrap().unwrap()
}
