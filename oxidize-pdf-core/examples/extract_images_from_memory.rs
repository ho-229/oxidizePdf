//! Example: Extract images from a PDF loaded into memory
//!
//! Demonstrates how to use ImageExtractor with an in-memory PDF document
//! using Cursor<Vec<u8>> instead of reading from a file.

use oxidize_pdf::parser::{PdfDocument, PdfReader};
use oxidize_pdf::{ExtractImagesOptions, ImageExtractor};
use std::io::Cursor;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Image Extraction from Memory Example ===\n");

    // Read PDF file into memory
    let input_pdf = "tests/fixtures/sample.pdf";
    println!("Loading PDF from: {}", input_pdf);
    
    let pdf_bytes = match std::fs::read(input_pdf) {
        Ok(bytes) => {
            println!("Loaded {} bytes into memory\n", bytes.len());
            bytes
        }
        Err(e) => {
            eprintln!("Error reading PDF file: {}", e);
            eprintln!("\nNote: This example requires a PDF file at tests/fixtures/sample.pdf");
            eprintln!("You can use any PDF file with images for testing.");
            return Err(e.into());
        }
    };

    // Create a Cursor (in-memory reader) from the bytes
    let cursor = Cursor::new(pdf_bytes);
    
    // Parse the PDF from the cursor
    println!("Parsing PDF from memory...");
    let reader = PdfReader::new(cursor)?;
    let document = PdfDocument::new(reader);
    println!("PDF parsed successfully\n");

    // Configure extraction options
    let options = ExtractImagesOptions {
        output_dir: PathBuf::from("examples/results/extracted_images_memory"),
        name_pattern: "page_{page}_image_{index}.{format}".to_string(),
        extract_inline: true,
        min_size: Some(10), // Skip images smaller than 10x10 pixels
        create_dir: true,
        ..Default::default()
    };

    // Create ImageExtractor with the in-memory document
    // This works because ImageExtractor is now generic over R: Read + Seek
    let mut extractor = ImageExtractor::new(document, options);
    
    println!("Extracting images...");
    match extractor.extract_all() {
        Ok(images) => {
            println!("Successfully extracted {} images:\n", images.len());

            for image in &images {
                println!(
                    "  Page {}, Image {}: {}x{} {} -> {}",
                    image.page_number + 1,
                    image.image_index + 1,
                    image.width,
                    image.height,
                    match image.format {
                        oxidize_pdf::graphics::ImageFormat::Jpeg => "JPEG",
                        oxidize_pdf::graphics::ImageFormat::Png => "PNG",
                        oxidize_pdf::graphics::ImageFormat::Tiff => "TIFF",
                        oxidize_pdf::graphics::ImageFormat::Raw => "RAW",
                    },
                    image.file_path.display()
                );
            }

            println!("\nImages extracted to: examples/results/extracted_images_memory/");
            println!("\nThis demonstrates that ImageExtractor can work with any Read + Seek source,");
            println!("not just files. Useful for web services, memory-constrained environments,");
            println!("or when working with encrypted/compressed data streams.");
        }
        Err(e) => {
            eprintln!("Error extracting images: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
