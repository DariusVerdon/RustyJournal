use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn read_file()-> std::io::Result<()> {
    let f = File::open("foo.txt")?;
    let f = BufReader::new(f);
    
    for line in f.lines(){
        println!("{}", line.unwrap());
    }
    Ok(())
}
