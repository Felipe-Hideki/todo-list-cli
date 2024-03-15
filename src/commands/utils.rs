use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

pub fn skip_elements(file: &mut File, size: usize)
{
    if size == 0
    {
        return;
    }
    let mut buf = [0u8; 1024];
    let mut elem_count = 0;
    
    loop {
        match file.read(&mut buf)
        {
            Ok(0) =>
            {
                break;
            },
            Ok(n) => 
            {
                for i in 0..n
                {
                    if elem_count == size
                    {
                        file.seek(SeekFrom::Current(i as i64 - n as i64 + 2)).unwrap();
                        return;
                    }

                    if buf[i] == b'|'
                    {
                        elem_count += 1;
                    }
                }
            },
            Err(e) =>
            {
                eprintln!("Failed to read file stream {e}\nretrying...");
                continue;
            }
        }
    };
}