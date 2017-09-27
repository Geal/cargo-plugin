extern crate toml;

use std::io::{self, Read, Write};
use std::fs::{File, read_dir};
use std::collections::BTreeMap;
use std::path::PathBuf;

use toml::Value;
use toml::value::Table;

fn main() {
    if let Err(e) = run() {
        println!("could not generate plugins:\n{:#?}", e);
    }
}

#[derive(Debug,Clone,PartialEq)]
struct Plugin {
    module_name: String,
    plugin_name: String,
    dependencies: Option<Table>,
}

fn run() -> io::Result<()> {

    let final_toml = {
    let mut cargo_toml = File::open("./Cargo.toml")?;
    let mut cargo_metadata = String::new();
    cargo_toml.read_to_string(&mut cargo_metadata)?;

    let mut value = cargo_metadata.parse::<Value>().expect("invalid Cargo.toml file");
    {
      let dependencies = &value["dependencies"];
      println!("got dependencies: {:#?}", dependencies);
    }

    let mut plugins: Vec<Plugin> = Vec::new();

    for dir_opt in read_dir("./src")? {
        let dir = dir_opt.unwrap();
        if dir.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
            let mut plugin_metadata_path = PathBuf::from(dir.path());
            plugin_metadata_path.push("metadata.toml");

            println!("loading  {:?}", plugin_metadata_path.to_str());

            let mut plugin_metadata_toml = File::open(plugin_metadata_path)?;
            let mut plugin_metadata = String::new();
            plugin_metadata_toml.read_to_string(&mut plugin_metadata)?;
            let plugin_value = plugin_metadata.parse::<Value>()
                .expect("invalid Cargo.toml file");

            println!("got metadata:\n{:#?}", plugin_value);

            plugins.push(Plugin {
                module_name: dir.file_name()
                    .to_str()
                    .expect("could not parse module name")
                    .to_string(),
                plugin_name: plugin_value.get("name")
                    .expect("no name in toml file")
                    .as_str()
                    .expect("could not parse name to string")
                    .to_string(),
                dependencies: plugin_value.get("dependencies")
                    .and_then(|deps| deps.as_table().cloned()),
            });
        }
    }

    println!("got plugin metadata:\n{:#?}", plugins);

    if let Some(table) = value.as_table_mut() {
      let deps = table.entry("dependencies".to_string()).or_insert(Value::Table(BTreeMap::new()));

      let mut plugin_dependencies: Vec<Table> = plugins.iter().map(|plugin| plugin.dependencies.clone())
        .filter(|dependencies| dependencies.is_some()).map(|dependencies| dependencies.unwrap()).collect();
      //deps.insert
      for dependencies in plugin_dependencies.drain(..) {
        deps.as_table_mut().map(|deps_table| deps_table.extend(dependencies.into_iter()));
      }

      println!("complete set of dependencies:\n{:?}", deps);
    }

    let final_cargo_toml = toml::to_string(&value).unwrap();
    println!("final file:\n{}", final_cargo_toml);

    final_cargo_toml
    };

    let mut file = File::create("./Cargo.toml")?;
    file.write_all(final_toml.as_bytes())?;

    Ok(())
}
