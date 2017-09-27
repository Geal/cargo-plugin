// this file is generated, do not edit manually
// (or edit it if you want, it's not like I can prevent you from doing it)

extern crate {{metadata.plugin_api_name}};

{{#each metadata.plugins as |plugin| ~}}
{{#each plugin.dependencies as |dependency| ~}}
#[macro_use] extern crate {{dependency}};
{{/each ~}}
{{/each ~}}


{{#each metadata.plugins as |plugin| ~}}
mod {{plugin.module_name}};
{{/each ~}}


{{#each metadata.plugins as |plugin| ~}}
pub use {{plugin.module_name}}::*;
{{/each ~}}


use {{metadata.plugin_api_name}}::PluginInformation;
use std::collections::hash_map::HashMap;

pub struct Plugins {
    pub list: HashMap<String, Box<PluginInformation>>,
}

pub fn plugins() -> Plugins {
    let mut h: HashMap<String, Box<PluginInformation>> = HashMap::new();

    {{#each metadata.plugins as |plugin| ~}}
    h.insert("{{plugin.plugin_name}}".to_string(), Box::new({{plugin.module_name}}::PLUGIN_METADATA));
    {{/each ~}}


    Plugins { list: h }
}
