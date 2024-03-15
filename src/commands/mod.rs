mod manager;
mod utils;

use manager::TodoListIterator;

use std::fs::File;
use std::io::{self, prelude::*, SeekFrom};
use std::mem::size_of_val;
use std::usize;



pub fn add(args: Vec<String>) -> Result<(), io::Error>
{
    let mut file = File::options().create(true).append(true).open("todo-list").expect("Failed to open file");
    let mut buffered_input = String::with_capacity(size_of_val(&args[..]) + 4 * args.len());

    for arg in args
    {
        if arg.contains('|')
        {
            eprintln!("Invalid 'todo' message {arg} as it contains '|' which is not allowed! ignoring it...");
            continue;
        }
        buffered_input += &(String::from("\n") + &arg + "|0");
    }
    file.write_all(buffered_input.as_bytes())?;
    Ok(())
}



pub fn mark_done(id: usize)
{
    let mut file = match File::options().write(true).read(true).open("todo-list")
    {
        Ok(val) =>
        {
            val
        },
        Err(e) =>
        {
            eprintln!("Failed to open file! {e}");
            return;
        }
    };

    utils::skip_elements(&mut file, id);

    file.write(&[b'1']).expect("Failed to write data!");
}

pub fn list()
{
    let file = match File::options().read(true).open("todo-list")
    {
        Ok(val) => val,
        Err(_) => 
        {
            eprintln!("Failed to open file, creating a new one");
            if let Ok(f) = File::create("todo-list")
            {
                f   
            }
            else 
            {
                panic!("Failed to create file!");
            }
        }
    };
    for (index, todo_elem) in TodoListIterator::new(file).enumerate()
    {
        println!("{}: {}", index, todo_elem.pretty_print());
    }
}

pub fn delete(id: usize) -> Result<(), io::Error>
{
    let mut file = File::options().write(true).read(true).open("todo-list")?;

    // Get the full size of the file
    let file_size = file.seek(SeekFrom::End(0))?;
    file.seek(SeekFrom::Start(0))?;

    // Get the position of the first char of the element at "id" element
    utils::skip_elements(&mut file, id);
    let id_pos = file.stream_position()?;

    // Skip the next element
    utils::skip_elements(&mut file, 1);
    let id_next_pos = file.stream_position()?;
    if id_next_pos == file_size
    {
        file.set_len(id_pos)?;
        return Ok(());
    }

    // Create a buffer with all the data after the "id" element
    let mut buf = String::with_capacity(file_size.saturating_sub(file.stream_position()?) as usize);
    file.read_to_string(&mut buf)?;

    // Truncate file
    file.set_len(id_pos)?;
    
    // Overwrite the "id" element with "buf"
    file.seek(SeekFrom::End(0))?;
    file.write_all(buf.as_bytes())?;

    
    Ok(())
}