use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Options {
    input: PathBuf,
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::from_args();

    let dimensions;
    let mut image_buffer;
    {
        let input = BufReader::new(File::open(&options.input)?);

        let decoder = png::Decoder::new(input);
        let (info, mut reader) = decoder.read_info()?;

        assert_eq!(
            info.color_type,
            png::ColorType::RGBA,
            "strip-alpha only works on 8-big RGBA PNGs"
        );
        assert_eq!(
            info.bit_depth,
            png::BitDepth::Eight,
            "strip-alpha only works on 8-bit RGBA PNGs"
        );

        dimensions = (info.width, info.height);
        image_buffer = vec![0; info.buffer_size()];

        reader.next_frame(&mut image_buffer)?;
    }

    for pixel in image_buffer.chunks_exact_mut(4) {
        pixel[3] = 255;
    }

    {
        let output = File::create(&options.output)?;
        let writer = BufWriter::new(output);

        let mut encoder = png::Encoder::new(writer, dimensions.0, dimensions.1);
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);
        let mut png_writer = encoder.write_header()?;
        png_writer.write_image_data(&image_buffer)?;
    }

    Ok(())
}
