use std::fs;

pub fn load_rom(path: String)->Result<Vec<u8>, std::io::Error>{
    let file = fs::read(path)?;
    Ok(file)
    
}
