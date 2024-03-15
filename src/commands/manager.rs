use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use colored::Colorize;

pub struct TodoElement
{
    msg: String,
    is_complete: bool
}

impl TodoElement
{
    pub fn pretty_print(&self) -> String
    {
        if self.is_complete
        {
            self.msg.strikethrough().red().to_string()
        }
        else
        {
            self.msg.clone()
        }
    }
}

impl From<String> for TodoElement
{
    fn from(value: String) -> Self 
    {
        let mut splitted = value.split('|');
        let (Some(msg), Some(is_complete)) = (splitted.next(), splitted.next())
        else
        {
            return Self {
                msg: String::new(),
                is_complete: false
            }
        };

        Self
        {
            msg: msg.to_string(),
            is_complete: (is_complete != "0")
        }
    }
}

impl ToString for TodoElement
{
    fn to_string(&self) -> String
    {
        format!("{}|{}\n", self.msg, self.is_complete)
    }
}

pub struct TodoListIterator
{
    file: File,
    buffer: [u8; 1024]
}

impl TodoListIterator
{
    pub fn new(file: File) -> Self
    {
        Self
        {
            file,
            buffer: [0u8; 1024]
        }
    }
}

impl TodoListIterator
{
    fn read_until_sep(&mut self) -> String
    {
        let mut string_buf = String::with_capacity(512);

        loop
        {
            match self.file.read(&mut self.buffer)
            {
                Ok(0) =>
                {
                    return string_buf;
                },
                Ok(n) =>
                {
                    for i in 0..n
                    {
                        string_buf.push(char::from(self.buffer[i]));
                        if self.buffer[i] == b'|'
                        {
                            self.file.seek(SeekFrom::Current(i as i64 - n as i64 + 1)).unwrap();
                            return string_buf;
                        }
                    }
                },
                Err(e) =>
                {
                    eprintln!("Couldnt read from file: {e}\nretrying...");
                    continue;
                }
            }
        }
    }
}

impl Iterator for TodoListIterator
{
    type Item = TodoElement;

    fn next(&mut self) -> Option<Self::Item>
    {
        let mut string_buf = self.read_until_sep();
        // Buffer to read the status and to skip the newline separator
        let mut buf = [0u8; 2];
        loop
        {
            match self.file.read(&mut buf)
            {
                Ok(0) =>
                {
                    return None;
                },
                Ok(_) =>
                {
                    string_buf.push(char::from(buf[0]));
                    break;
                },
                Err(e) =>
                {
                    eprintln!("Couldnt read todo status: {e}\nretrying...");
                    continue;
                }
            }
        };
        Some(TodoElement::from(string_buf))
    }
}