use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sp_core::H256;
use tempfile::tempdir;

const SOURCE: &str = "Source";

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(tag = "type")]
pub(crate) enum Source {
    Wasm { github_repo: String, hash: String },
    Rpc { block: H256 },
}

// Add `Source` info to png file as a zTXt chunk
pub(crate) fn save_source_info(path: &Path, source: &Source) -> Result<()> {
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let mut reader = decoder.read_info().unwrap();
    // If the text chunk is before the image data frames, `reader.info()` already contains the text.
    let in_info = reader.info();

    let tmp_dir = tempdir().unwrap();
    let out_path = tmp_dir.path().join("qr.apng");
    let file = File::create(&out_path).unwrap();
    let w = &mut BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, in_info.width, in_info.height);
    encoder.set_color(in_info.color_type);
    encoder.set_depth(in_info.bit_depth);
    if let Some(palette) = in_info.palette.clone() {
        encoder.set_palette(palette);
    }
    if let Some(animation) = in_info.animation_control {
        encoder.set_animated(animation.num_frames, animation.num_plays)?;
    }
    if let Some(frame) = in_info.frame_control {
        encoder.set_frame_delay(frame.delay_num, frame.delay_den)?;
    }
    encoder.add_ztxt_chunk(SOURCE.to_string(), serde_json::to_string(source)?)?;
    encoder.set_compression(png::Compression::Best);

    let mut writer = encoder.write_header().unwrap();

    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    while let Ok(info) = reader.next_frame(&mut buf) {
        let bytes = &buf[..info.buffer_size()];
        writer.write_image_data(bytes)?; // Save
    }

    writer.finish()?;
    // Replace the original file with the patched one.
    fs::rename(out_path, path)?;
    Ok(())
}

// Read source metadata from zTXt chunks
pub(crate) fn read_png_source(path: &Path) -> Result<Option<Source>> {
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let reader = decoder.read_info().unwrap();
    // If the text chunk is before the image data frames, `reader.info()` already contains the text.
    for text_chunk in &reader.info().compressed_latin1_text {
        if text_chunk.keyword == SOURCE {
            let text = text_chunk.get_text()?;
            let source: Source = serde_json::from_str(&text)?;
            return Ok(Some(source));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn save_and_read_img_source() {
        let original_png = Path::new("./src/for_tests/no_source/image.png");
        let test_png = Path::new("./src/for_tests/no_source/testing.png");
        fs::copy(original_png, test_png).unwrap();

        let source = Source::Rpc {
            block: H256::default(),
        };
        save_source_info(test_png, &source).unwrap();
        let result = read_png_source(test_png).unwrap().unwrap();
        assert_eq!(source, result);

        fs::remove_file(test_png).unwrap();
    }

    #[test]
    fn no_source() {
        let path = Path::new("./src/for_tests/no_source/image.png");
        let result = read_png_source(path).unwrap();
        assert!(result.is_none());
    }
}
