use crate::error::Error;

use image::io::Reader as ImageReader;
use image::codecs::jpeg::JpegEncoder;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::io::{stdout, stdin};
use std::f64::INFINITY;
use std::io::Write;

// use turbojpeg;



pub fn compress_image(image_name: String) -> Result<String, Error> {
	println!("image_name = {}", image_name);
	// let img = ImageReader::open("/home/a/Documents/originalDog.jpeg")?.decode()?;
	// let weights: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
	// let values: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
	// let raw_val: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
	// let roundup: Arc<Mutex<Vec<bool>>> = Arc::new(Mutex::new(Vec::new()));
	// let index: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
	// let mut buffer = File::create("dog2.jpeg")?;
	// let mut jpeg_encoder = JpegEncoder::new_with_quality(buffer, 92);
	// jpeg_encoder.encode(img.as_bytes(), img.width(), img.height(), image::ColorType::Rgb8, Some(&|p| {
	// 	let i = index.lock().unwrap().clone();
	// 	let mut weight = 255;
	// 	if (i % 3) as f32 != 0.0 {
	// 		weight = ((p%1.0)*100.0).abs() as u8;
	// 		if weight >= 50 {
	// 			weight -= 50;
	// 			roundup.lock().unwrap().push(true);
	// 		} else {
	// 			weight = 50 - weight;
	// 			roundup.lock().unwrap().push(false);
	// 		}
	// 		weights.lock().unwrap().push(weight+1);
	// 		values.lock().unwrap().push(p.round() as u8 & (1 << 0) as u8);
	// 		raw_val.lock().unwrap().push(p);
	// 	}
	// 	*index.lock().unwrap() += 1;
	// 	0.0
	// }));
	// println!("weights = {}", weights.lock().unwrap().len());

	// // for i in 0..200 {
	// // 	println!("v = {}, rv = {}, w = {}", values.lock().unwrap()[i], raw_val.lock().unwrap()[i], weights.lock().unwrap()[i]);
	// // }



	// let mut cover_object: Vec<u8> = values.lock().unwrap().clone(); //cover_object vector 
	// let mut rounds: Vec<bool> = roundup.lock().unwrap().clone();
	// // let mut full_image: Vec<u8> = Vec::new(); // the entire image vector
	// let mut cover_weights: Vec<u8> = weights.lock().unwrap().clone(); //weights for each cover_object bit
	// // let jpeg_data = std::fs::read("./dog.jpeg").expect("failed to read image");


	// let mut s = String::new(); //Create message string
 //    print!("Please enter some text: "); //Print to console
 //    let _=stdout().flush(); //new line for console
 //    stdin().read_line(&mut s).expect("Did not enter a correct string"); //Grab message
 //    s = s.trim().to_string();

	// let mut message = Vec::<u64>::new(); //create message vector
	// for m in s.bytes() { //loop through bytes of the message string
	// 	let mut binary = format!("{m:b}"); //convert bytes to bits
	// 	for _ in 0..(8-binary.len()) { //bad binary to proper 8 bit form
	// 		let filler: String = String::from("0"); //create a 0 string
	// 		binary = filler+&binary //pad binary
	// 	}
	// 	for i in binary.chars() { //grab each bit of the binary string
	// 		message.push(i.to_string().parse::<u64>().unwrap()); //parse binary into a u64 int and push to message vector
	// 	}
	// }
	// print!("message: ");
	// for i in 0..message.len() {
	// 	print!("{}, ", message[i]);
	// }
	// println!(";");
	
	// while cover_object.len()%message.len() != 0 { //trim cover_object to be a multiple of the message vector
	// 	cover_object.pop(); //pop the end off the cover_object
	// }
	// println!("cover_object.len() = {}", cover_object.len());
	// println!("message.len() = {}", message.len());
	// let sub_width = cover_object.len()/message.len(); //rate of the encoding or the width of the sub matrix H
 //    let sub_height: usize = 4; //performance parameter
 //    let h = 2_u64.pow(sub_height as u32); //2^h
 //    let mut sub_h: Vec<Vec<u64>> = Vec::new(); //create the sub_h or h_hat vector
 //    for i in 0..sub_height {
 //    	sub_h.push(Vec::new());
 //    	for _ in 0..sub_width {
 //    		if rand::random() { //randomly push a zero or one
 //    			sub_h[i].push(1); 
 //    		} else {
 //    			sub_h[i].push(0);
 //    		}
 //    	} 
 //    }

 //    let mut sub_ch: Vec<Vec<u64>> = Vec::new(); //create the column oriented sub_h or h_hat
 //    for i in 0..sub_width {
 //    	sub_ch.push(Vec::new());
 //    	for ii in 0..sub_height {
 //    		sub_ch[i].push(sub_h[ii][i]);
 //    	}
 //    }

 //    let mut ph_hat: Vec<Vec<u64>> = Vec::new(); //A vector of vectors, the first of which contains the int format for each column of sub_h or h_hat, the remaining vectors contain the trimed columns based on the extended H. 
 //    for i in 0..sub_height { //The number of trimed column blocks
 //    	ph_hat.push(Vec::new());
 //    	for ii in 0..sub_width {
 //    		ph_hat[i].push(0);
 //    		for iii in 0..(sub_height-i) { //the number of bits per trimmed column
 //    			ph_hat[i][ii] += sub_ch[ii][iii]*2_u64.pow(iii as u32); //binary to int
 //    		}
 //    	}
 //    }
 
 //    let ext_height = message.len(); //extended matrix H height
 //    let ext_width = cover_object.len(); //extended matrix W width
 //    println!("sub_width = {}", sub_width);
 //    let b = ext_width/sub_width; //Number of copies of sub_h or h_hat in the extended matrix. Includes trimmed blocks.


 //    let mut ext_h: Vec<Vec<u64>> = Vec::new(); //extended matrix
 //    for i in 0..(ext_height) {
 //    	ext_h.push(Vec::new());
 //    	for _ in 0..ext_width {
 //    		ext_h[i].push(0);
 //    	}
 //    }

 //    let mut ext_ch: Vec<Vec<u64>> = Vec::new(); //extended matrix column oriented
 //    for i in 0..(ext_width) {
 //    	ext_ch.push(Vec::new());
 //    	for _ in 0..ext_height {
 //    		ext_ch[i].push(0);
 //    	}
 //    }

 //    let mut row = 0;
 //    let mut column = 0;
 //    'B: for _ in 0..(ext_width/sub_width) { //Builds the extended matrix
 //    	'H: for ii in 0..sub_height {
 //    		for iii in 0..sub_width {
 //    			if row+ii >= ext_height {
 //    				break 'H
 //    			}
 //    			if column+iii >= ext_width {
 //    				break 'B
 //    			}
 //    			ext_h[row+ii][column+iii] = sub_h[ii][iii];
 //    		}
 //    	}
 //    	row += 1;
	// 	column = column+sub_width;
 //    }
    
 //    for i in 0..ext_h[0].len() { //Builds the column oriented extended matrix
 //    	for ii in 0..ext_h.len() {
 //    		ext_ch[i][ii] = ext_h[ii][i];
 //    	}
 //    }

	// fn matrix_multi(s: &mut Vec<u64>, x: &mut Vec<u8>, ch: &Vec<Vec<u64>>, ext_height: usize) { //multiplys a vector of length equal to that of the cover object against the extended matrix. The result is a syndrom the length of the message.
	// 	for _ in 0..ext_height {
	// 		s.push(0);
	// 	}
	// 	for i in 0..ch.len() {
	// 		for ii in 0..ch[0].len() {
	// 			// if (i == (ch.len()-1)) {
	// 			// 	println!("ch[{}][{}] = {}, x[{}] = {}", i, ii, ch[i][ii], ii, x[ii]);
	// 			// }
	// 			s[i] = (s[i]+((x[ii] as u64*ch[i][ii])%2))%2;
	// 		}
	// 	}
	// 	for i in 0..ext_height {
	// 		s[i] = s[i]%2;
	// 	}
	// }



	// let mut path: Vec<Vec<u64>> = Vec::new(); //path vector of vectors contains a vector of each state for each column.
	// for i in 0..cover_object.len() {
	// 	path.push(Vec::new());
	// 	for _ in 0..h {
	// 		path[i].push(0);
	// 	}
	// }
	// let mut wght: Vec<f64> = Vec::new(); //contains the cost per path
	// wght.push(0.0);
	// for _ in 1..h {
	// 	wght.push(INFINITY);
	// }
	// let mut y: Vec<u8> = Vec::new(); //stego cover object
	// for _ in 0..cover_object.len() {
	// 	y.push(0);
	// }
	// let mut indx = 0;
	// let mut indm = 0;

	// //FORWARD RUN
	// for _ in 1..((b+1) as usize) { //For each copy of sub_h in ext_h
	// 	for j in 0..((sub_width) as usize) { //for each column
	// 		let mut newwght: Vec<f64> = Vec::new();
	// 		for _ in 0..h {
	// 			newwght.push(INFINITY);
	// 		}
	// 		for k in 0..(h as usize) { //for each state 
	// 			let mut phindex = 0; 
	// 			if (indm+sub_height) > b { //Decides if the current column is a trimed version of sub_h or h_hat
	// 				phindex = (indm+sub_height)-b;
	// 			}
	// 			let w0 = wght[k] + ((cover_object[indx]*cover_weights[indx]) as f64); //weight of not adding the current column of sub_h or h_hat
	// 			let w1 = wght[((k as u64)^ph_hat[phindex][(j%sub_width) as usize]) as usize] + ((((1+cover_object[indx])%2)*cover_weights[indx]) as f64); //weight of adding the current column of sub_h or h_hat
	// 			path[indx][k] = if w1 < w0 { //recordes the available paths for this state
	// 				1
	// 			} else {
	// 				0
	// 			};
	// 			// println!("j = {}, w0 = {}, w1 = {}, p[{}][{}] = {}", j, w0, w1, indx, k, path[indx][k]);
	// 			newwght[k] = w0.min(w1); //decides if adding or not addingh the column of h_hat was cheeper
	// 		}
	// 		indx += 1;
	// 		wght = newwght;
	// 	}
	// 	for j in 0..h/2 {
	// 		// println!("{}, {}", indm, message[indm]);
	// 		wght[j as usize] = wght[((2*j) + message[indm]) as usize]; // squashes the weights by half taking either the even node or the odd node based on the message bit
	// 	}
	// 	for j in h/2..h {
	// 		wght[j as usize] = INFINITY; //zeros out the second half after the squash
	// 	}
	// 	indm += 1;
	// }
	// let embeding_cost = wght[0]; 
	// println!("embeding cost = {}", embeding_cost);

	// //BACKWARDS RUN
	// let mut state: u64 = message[(indm-1) as usize]; //current state of the trellis //message[(indm-1) as usize]
	// for _ie in 1..((b+1) as usize) { //for each copy of sub_h or h_hat
	// 	indm -= 1;
	// 	// let _i = b-ie; // To go backwards
	// 	for je in 1..((sub_width+1) as usize) { //for each column
	// 		indx -= 1;
	// 		let j = sub_width-je; // To go backwards
	// 		y[indx] = path[indx][state as usize] as u8; //set the stego object bit for this state
	// 		if y[indx] != cover_object[indx] {
	// 			println!("differ");
	// 			if rounds[indx] {
	// 				rounds[indx] = false
	// 			} else {
	// 				rounds[indx] = true
	// 			}

	// 		}
	// 		let mut phindex = 0;
	// 		if (indm+sub_height) > b { //decides if we need to use a trimed copy of h_hat or sub_h
	// 			phindex = (indm+sub_height)-b; 
	// 		}
	// 		state = state^((y[indx] as u64*ph_hat[phindex][(j%sub_width) as usize])); //updates the state based on cheepest choice
	// 	}
	// 	if indm == 0 {
	// 		break
	// 	}
	// 	state = (2*state + message[indm-1 as usize]) % h; //updates the state to account for the pruning
	// }

	// let mut syndrom = Vec::new();
	// matrix_multi(&mut syndrom, &mut y,  &ext_h, ext_height);
	// assert!(syndrom == message); // Assert virtubi succeded

	// //WRITE IMAGE
	// let index: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
	// let indx: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
	// let diffind: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));
	// let mut buffer = File::create("dog2.jpeg")?;
	// let test_round = roundup.lock().unwrap().clone();
	// let mut jpeg_encoder = JpegEncoder::new_with_quality(buffer, 92);
	// jpeg_encoder.encode(img.as_bytes(), img.width(), img.height(), image::ColorType::Rgb8, Some(&|p| {
	// 	let i = index.lock().unwrap().clone();
	// 	let mut weight = 255;
	// 	if (i % 3) as f32 != 0.0 {
	// 		let ind = *indx.lock().unwrap() as usize;
	// 		if rounds[ind] != test_round[ind] {
	// 			println!("+iffer {}, {}, {}, {}, {}, {}", ind, y[ind], cover_object[ind], rounds[ind], test_round[ind], p);
	// 			diffind.lock().unwrap().push(ind);

	// 		}
	// 		if rounds[*indx.lock().unwrap() as usize] {
	// 			*indx.lock().unwrap() += 1;
	// 			*index.lock().unwrap() += 1;
	// 			return p.ceil()
	// 		} else {
	// 			*indx.lock().unwrap() += 1;
	// 			*index.lock().unwrap() += 1;
	// 			return p.floor()
	// 		}
	// 	}
	// 	*index.lock().unwrap() += 1;
	// 	p.round()
	// }));

	// let diffind = diffind.lock().unwrap().clone();

	// //READ IMAGE TO CHECK SYNDROM
	// let stego_ob: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
	// let mut buffer = File::create("garbage.jpeg")?;
	// let img = ImageReader::open("dog2.jpeg")?.decode()?;
	// let mut jpeg_encoder = JpegEncoder::new_with_quality(buffer, 100);
	// let index: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
	// let indx: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
	// jpeg_encoder.encode(img.as_bytes(), img.width(), img.height(), image::ColorType::Rgb8, Some(&|p| {
	// 	let i = index.lock().unwrap().clone();
	// 	if (i % 3) as f32 != 0.0 {
	// 		let bit = p as u8 & (1 << 0) as u8;
	// 		// if diffind.contains(&(indx.lock().unwrap().clone() as usize)) {
	// 		// 	println!("p = {}", p);
	// 		// }
	// 		let ind = indx.lock().unwrap().clone() as usize;
	// 		// println!("+iffer {}, {}, {}, {}, {}", ind, y[ind], cover_object[ind], bit, p);

	// 		// if bit != y[ind] {
	// 			// print!("{},", ind);
	// 		// }
	// 		*indx.lock().unwrap() += 1;
	// 		stego_ob.lock().unwrap().push(bit);
	// 	}
	// 	*index.lock().unwrap() += 1;
	// 	0.0
	// }));

	// let mut stego_ob = stego_ob.lock().unwrap().clone();
	// matrix_multi(&mut syndrom, &mut stego_ob,  &ext_h, ext_height);
	// let mut differs = 0;
	// for i in 0..stego_ob.len() {
	// 	if stego_ob[i] != y[i] {
	// 		differs += 1;
	// 	}
	// }
	// println!("differs = {}", differs);
	// // assert!(y == stego_ob);
	// assert!(syndrom == message); // Assert virtubi succeded













	use std::io;
use std::io::prelude::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}


















	// println!("rounds = {}", rounds.lock().unwrap());

	// jpeg_encoder.encode(img.as_bytes(), img.width(), img.height(), image::ColorType::Rgb8, Some(&|p| {
	// 	let i = index.lock().unwrap().clone();
	// 	println!("index = {}", i);
	// 	let mut weight = 100000;
	// 	let mut value = 0;
	// 	if (i % 3) as f32 != 0.0 {
	// 		println!("CR or CB");
	// 		if p != 0.0 {
	// 			weight = ((p%1.0)*100.0).abs() as i32;
	// 			if (weight >= 50) {
	// 				weight -= 50;
	// 				value = 1;
	// 			} else {
	// 				weight = 50 - weight;
	// 			}
	// 		} 
	// 	} else {
	// 		return p.round()
	// 	}
	// 	println!("weight = {}", weight);
	// 	weights.lock().unwrap().push(weight);
	// 	values.lock().unwrap().push(value);
	// 	*index.lock().unwrap() += 1;
	// 	0.0
	// }));

	// use image::{RgbImage, Rgb};
	// use rand::Rng;


	// let a = 8;
	// let b = 8;
	// let mut img = RgbImage::new(a, b);
	// for x in 0..a {
	// 	for y in 0..b {
	// 		let mut rng = rand::thread_rng();
 //    		let g: u8 = rng.gen();
	// 		// println!("g = {}", g);
	// 		img.put_pixel(x, y, Rgb([200, g, 100]));
	// 	}
	// }
	




	let img = ImageReader::open("/home/a/app/dog5.jpeg")?.decode()?;
	let mut buffer = File::create("/home/a/app/dog6.jpeg")?;
	let mut jpeg_encoder = JpegEncoder::new_with_quality(buffer, 100);
	let index: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
	let values: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
	jpeg_encoder.encode(img.as_bytes(), img.width(), img.height(), image::ColorType::Rgb8, None);
	

	pause();
	println!("------------------------------------------------------");
	// use std::io::BufReader;
	// use image::io::Reader;
	// let reader = Reader::open("/home/a/app/dog5.jpeg")?;
	// println!("reader.format = {}", reader.format().unwrap() == image::ImageFormat::Jpeg);
	let img = ImageReader::open("/home/a/app/dog6.jpeg")?.decode()?;
	let mut buffer = File::create("/home/a/app/dog7.jpeg")?;
	let mut jpeg_encoder = JpegEncoder::new_with_quality(buffer, 100);
	let index: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
	let valuex: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
	jpeg_encoder.encode(img.as_bytes(), img.width(), img.height(), image::ColorType::Rgb8, None);

	// let v1 = values.lock().unwrap().clone();
	// let v2 = valuex.lock().unwrap().clone();
	// let mut True = 0;
	// for i in 0..v1.len() {
	// 	if v1[i] == v2[i] {
	// 		// println!("v1[{}] = {}, v2[{}] = {}", i, v1[i], i, v2[i]);
	// 		// panic!("NEQ");
	// 		True += 1
	// 	}
	// }
	// println!("DTC COEEFS = {}/{}", True, v1.len());

	// let mut test = 0;

	// let mut valuex = Vec::new();

	// use image::{GenericImageView, Pixel, Primitive};
	// use num_traits::cast::ToPrimitive;

	// fn rgb_to_ycbcr<P: Pixel>(pixel: P, mut test: u64) -> (u8, u8, u8) {

	//     let [r, g, b] = pixel.to_rgb().0;
	//     let max: f32 = P::Subpixel::DEFAULT_MAX_VALUE.to_f32().unwrap();
	//     let r: f32 = r.to_f32().unwrap();
	//     let g: f32 = g.to_f32().unwrap();
	//     let b: f32 = b.to_f32().unwrap();

	//     // Coefficients from JPEG File Interchange Format (Version 1.02), multiplied for 255 maximum.
	//     let y = 76.245 / max * r + 149.685 / max * g + 29.07 / max * b;
	//     let cb = -43.0185 / max * r - 84.4815 / max * g + 127.5 / max * b + 128.;
	//     let cr = 127.5 / max * r - 106.7685 / max * g - 20.7315 / max * b + 128.;
	    // let y: f32 = 16.0 + 65.738*r/256.0 + 129.057*g/256.0 + 25.064*b/256.0;
	    // let cb: f32 = 128.0-37.945*r/256.0 - 74.494*g/256.0 + 112.439*b/256.0;
	    // let cr: f32 = 128.0+112.439*r/256.0 - 94.154*g/256.0 - 18.285*b/256.0;
	//     if test * 3 == 12 {
	//     	println!("y = {}", y);
	//     	println!("cb = {}", cb);
	//     	println!("cr = {}", cr);
	//     }
	//     test += 1;

	//     (y.round() as u8, cb.round() as u8, cr.round() as u8)
	// }

	// fn pixel_at_or_near<I: GenericImageView>(source: &I, x: u32, y: u32) -> I::Pixel {
	//     if source.in_bounds(x, y) {
	//         source.get_pixel(x, y)
	//     } else {
	//         source.get_pixel(x.min(source.width() - 1), y.min(source.height() - 1))
	//     }
	// }

	// fn copy_blocks_ycbcr<I: GenericImageView>(
	//     source: &I,
	//     x0: u32,
	//     y0: u32,
	//     yb: &mut [u8; 64],
	//     cbb: &mut [u8; 64],
	//     crb: &mut [u8; 64],
	//     mut test: u64
	// ) {
	//     for y in 0..8 {
	//         for x in 0..8 {
	//             let pixel = pixel_at_or_near(source, x + x0, y + y0);
	//             let (yc, cb, cr) = rgb_to_ycbcr(pixel, test);

	//             yb[(y * 8 + x) as usize] = yc;
	//             cbb[(y * 8 + x) as usize] = cb;
	//             crb[(y * 8 + x) as usize] = cr;
	//             if test * 3 == 12 {
	//             	let p = pixel.to_rgb().0;
	//             	println!("r = {}", p[0].to_f32().unwrap());
	//             	println!("g = {}", p[1].to_f32().unwrap());
	//             	println!("b = {}", p[2].to_f32().unwrap());
 //            		println!("yc = {}", yc);
 //            		println!("cb = {}", cb);
 //            		println!("cr = {}", cr);
 //            	}
 //            	test += 1;
	//         }
	//     }
	// }

	// let mut yblock = [0u8; 64];
 //   	let mut cb_block = [0u8; 64];
	// let mut cr_block = [0u8; 64];
	// for y in (0..image.height()).step_by(8) {
 //        for x in (0..image.width()).step_by(8) {
 //            // RGB -> YCbCr
 //            copy_blocks_ycbcr(&image, x, y, &mut yblock, &mut cb_block, &mut cr_block, test);

 //            // // Level shift and fdct
 //            // // Coeffs are scaled by 8
 //            // transform::fdct(&yblock, &mut dct_yblock);
 //            // transform::fdct(&cb_block, &mut dct_cb_block);
 //            // transform::fdct(&cr_block, &mut dct_cr_block);

 //            // // Quantization
 //            for i in 0usize..64 {
 //            	if test * 3 == 12 {
 //            		println!("y = {}", yblock[i]);
 //            		println!("cb = {}", cb_block[i]);
 //            		println!("cr = {}", cr_block[i]);
            		
 //            	}
 //            	test += 1;
 //            	valuex.push(yblock[i]);
 //            	valuex.push(cb_block[i]);
 //            	valuex.push(cr_block[i]);
 //            //     dct_yblock[i] =
 //            //         quantization_decider((dct_yblock[i] / 8) as f32 / f32::from(self.tables[0][i]), yblock[i]) as i32;
 //            //     dct_cb_block[i] = quantization_decider((dct_cb_block[i] / 8) as f32 / f32::from(self.tables[1][i]), cb_block[i]) as i32;
 //            //     dct_cr_block[i] = quantization_decider((dct_cr_block[i] / 8) as f32 / f32::from(self.tables[1][i]), cr_block[i]) as i32;
 //            //     if test < 1 {
 //            //         println!("dct y = {}", dct_yblock[i]);
 //            //         println!("dct cb = {}", dct_cb_block[i]);
 //            //         println!("dct cr = {}", dct_cr_block[i]);
 //            //         println!("y = {}", yblock[i]);
 //            //         println!("cb = {}", cb_block[i]);
 //            //         println!("cr = {}", cr_block[i]);
 //            //     }
 //            //     test = 1;
 //            }

 //            // let la = &*self.luma_actable;
 //            // let ld = &*self.luma_dctable;
 //            // let cd = &*self.chroma_dctable;
 //            // let ca = &*self.chroma_actable;

 //            // y_dcprev = self.writer.write_block(&dct_yblock, y_dcprev, ld, la)?;
 //            // cb_dcprev = self.writer.write_block(&dct_cb_block, cb_dcprev, cd, ca)?;
 //            // cr_dcprev = self.writer.write_block(&dct_cr_block, cr_dcprev, cd, ca)?;
 //        }
 //    }


	// let img = ImageReader::open("/home/a/app/dog5.jpeg")?.decode()?;
	// let mut buffer = File::create("/home/a/app/dog6.jpeg")?;
	// let mut jpeg_encoder = JpegEncoder::new_with_quality(buffer, 100);
	// jpeg_encoder.encode(img.as_bytes(), img.width(), img.height(), image::ColorType::Rgb8, None);
	println!("-----------------------");
	let mut images = Vec::new();
	// images.push(("0", ImageReader::open("/home/a/Documents/originalDog.jpeg")?.decode()?));
	images.push(("1", ImageReader::open("/home/a/app/dog6.jpeg")?.decode()?));
	images.push(("2", ImageReader::open("/home/a/app/dog7.jpeg")?.decode()?));
	// images.push(("3", ImageReader::open("/home/a/app/dog6.jpeg")?.decode()?));

	for i in 0..images.len()-1 {
		let (name, image) = &images[i];
		let debug_output = format!("image dubug output = {:?}", image);
		let image_bytes = image.as_bytes();
		for x in 0..images.len() {
			if i != x {
				let (second_name, second_image) = &images[x];
				let second_image_bytes = second_image.as_bytes();
				let mut True = 0;
				let mut False = 0;
				println!("indexs:");
				for i in 0..image_bytes.len() {
					let byte1 = image_bytes[i];
					let byte2 = second_image_bytes[i];
					if byte1 == byte2 {
						True += 1;
					} else {
						if False < 100 {
							print!("{},", i);
						}
						False += 1
					}
				}
				println!(";");
				println!("{} VS {} = {}/{}", name, second_name, True, image_bytes.len());
			}
		}
	}
	Ok("Completed with out problems".to_string())
}