use std::fs;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use std::error::Error;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<(), Box<dyn Error>> {
    let bytes: Vec<u8> = fs::read(args.file_path.clone())?;
    let mut image = Png::try_from(bytes.as_slice())?;
    let chunk = Chunk::new(
        ChunkType::from_str(&args.chunk_type)?,
        args.message.into_bytes(),
    );
    image.append_chunk(chunk);

    let out = match args.output {
        Some(o) => o,
        None => args.file_path,
    };

    fs::write(out, image.as_bytes())?;
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<(), Box<dyn Error>> {
    let bytes: Vec<u8> = fs::read(args.file_path)?;
    let image = Png::try_from(bytes.as_slice())?;
    if let Some(chunk) = image.chunk_by_type(&args.chunk_type) {
        println!("{}", chunk.data_as_string()?);
    }
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<(), Box<dyn Error>> {
    let bytes: Vec<u8> = fs::read(args.file_path.clone())?;
    let mut image = Png::try_from(bytes.as_slice())?;
    image.remove_chunk(&args.chunk_type)?;
    fs::write(&args.file_path, image.as_bytes())?;
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<(), Box<dyn Error>> {
    let bytes: Vec<u8> = fs::read(args.file_path)?;
    let image = Png::try_from(bytes.as_slice())?;
    for chunk in image.chunks().iter() {
        println!("{}", chunk);
    }
    Ok(())
}
