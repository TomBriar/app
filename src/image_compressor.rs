use crate::error::Error;

use image::io::Reader as ImageReader;
use image::codecs::jpeg::JpegEncoder;
use std::fs::File;
use turbojpeg;


pub fn compress_image(image_name: String) -> Result<String, Error> {
	println!("image_name = {}", image_name);
	let img = ImageReader::open("/home/a/app/dog2-2.jpeg")?.decode()?;
	println!("img = {}", img.height());
	//IMAGE COMP
	let buffer = File::create("dog2-2.jpeg")?;
	let mut jpeg_encoder = JpegEncoder::new_with_quality(buffer, 100);
	jpeg_encoder.encode(img.as_bytes(), img.width(), img.height(), image::ColorType::Rgb8).unwrap();
	//IMAGE COMP
	//TURBO COMP
	// let img = img.as_rgb8().unwrap();
	// let jpeg_data = turbojpeg::compress_image(&img, 100, turbojpeg::Subsamp::Sub2x2)?;
	// std::fs::write("/home/a/Documents/turbodog.jpeg", jpeg_data)?;
	//TURBO COMP
	let images = vec![
		ImageReader::open("/home/a/app/dog2.jpeg")?.decode()?,
		ImageReader::open("/home/a/app/dog2-1.jpeg")?.decode()?,
		ImageReader::open("/home/a/app/dog2-2.jpeg")?.decode()?,
		// ImageReader::open("/home/a/Documents/doggimp.jpeg")?.decode()?,
		// ImageReader::open("/home/a/Documents/turbodog.jpeg")?.decode()?,
		// ImageReader::open("/home/a/Documents/libjpegdog.jpeg")?.decode()?,
	];

	for (i, image) in images.iter().enumerate() {
		let debug_output = format!("image dubug output = {:?}", image);
		println!("debug_output[:200] = {}", &debug_output[..200]);
		let image_bytes = image.as_bytes();
		for (x, second_image) in images.iter().enumerate().skip(i) {
			let second_image_bytes = second_image.as_bytes();
			let mut true_count = 0;
			let mut false_count = 0;
			for i in 0..image_bytes.len() {
				let byte1 = image_bytes[i];
				let byte2 = second_image_bytes[i];
				if byte1 == byte2 {
					true_count += 1;
				} else {
					false_count += 1;
				}
			}
			println!("{} VS {} = {}T/{}F", i, x, true_count, false_count);
		}
	}
	Ok("Completed with out problems".to_string())
}
