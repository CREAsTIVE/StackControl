use std::{env, fs::File, io::Write};

use stack_control::{bytecode::commands::core::bind_default_commands, compiletime::command_map::CommandMap};

#[derive(serde::Serialize)]
struct CommandMeta<'m> {
    key: char,
    aliases: &'m Vec<String>,
    description: &'m str,
}

fn main() -> Result<(), i32> {
    let mut command_map = CommandMap::new();
    bind_default_commands(&mut command_map);
    let metadata: Vec<CommandMeta> = command_map.collection.values()
        .map(|c| CommandMeta {
            key: c.meta.key,
            aliases: &c.meta.aliases,
            description: &c.meta.description.trim()
        }).collect();
    
    let string = serde_json::to_string(&metadata).or_else(|e| {
        println!("Serialization error: {:?}", e.to_string());
        Err(1)
    })?;

    let filename = env::args().skip(1).next().ok_or_else(|| {
        println!("Output path not specified");
        -2
    })?;

    let mut file = File::create(&filename)
    .or_else(|e| {
        println!("Can't create a file ({:?}): {:?}", &filename, e.to_string());
        Err(-3)
    })?;

    file.write_all(string.as_bytes()).or_else(|e| {
        println!("Can't write to a file: {:?}", e.to_string());
        Err(-4)
    })?;

    Ok(())
}
