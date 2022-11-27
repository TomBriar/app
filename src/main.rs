extern crate app;
extern crate bitcoincore_rpc;

use app::transaction::serialize;
use app::transaction::deserialize;
use app::error::Error;
use app::stego::{stego, matrix_multi};



#[cfg(feature="gui")]
use app::gui::gui;

use std::fs::File;
use std::process;
use std::io::{stdin, stdout, Write, Read, BufReader};
use std::sync::{Mutex, Arc};
use bitcoincore_rpc::{Auth, Client, RpcApi};
use image::codecs::jpeg::JpegEncoder;
use image::io::Reader as ImageReader;
use hex::FromHex;
use bitcoin::Transaction;
use bitcoin::psbt::serialize::Deserialize;

fn pretty_unwrap<T>(msg: &str, res: Result<T, Error>) -> T {
    match res {
        Ok(r) => r,
        Err(error) => {
            print!("{}: ", msg);
            match error {
                Error::CompressingTransactionError => {
                    println!("Could not Compress Transaction: {}", error);
                },
                // Otherwise just print the error
                e => println!("{}", e)
            }
            process::exit(1);
        }
    }
}

pub fn main() { 

	//STEP 0 parse bitcoin.conf
	let mut rpcport = String::new();
	let mut rpcuser = String::new();
	let mut rpcpass = String::new();
	//TODO: proper home dir
	let dot_bitcoin = "/home/tom/.bitcoin".to_string();
    let mut file = File::open(dot_bitcoin+"/bitcoin.conf").expect("Read File");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Read File");
    let lines = contents.split('\n');
    for line in lines {
        let property = line.split('=');
        let property_vec = property.collect::<Vec<&str>>();
        if property_vec[0] == "rpcuser" {
            rpcuser = property_vec[1].to_string();
        } else if property_vec[0] == "rpcpassword" {
            rpcpass = property_vec[1].to_string();
        } else if property_vec[0] == "rpcport" {
            rpcport = property_vec[1].to_string();
        }
    }
    let rpc = Client::new(
		&("http://localhost:".to_owned()+&rpcport),
  		Auth::UserPass(
  			rpcuser,
            rpcpass
       	)
    ).unwrap();

	//STEP 1 insert transaction and compress
	// let mut raw_transaction = String::new(); //Create message string
    // print!("Please enter your raw transaction: "); //Print to console
    // let _=stdout().flush(); //new line for console
    // stdin().read_line(&mut raw_transaction).expect("Read Terminal Input"); //Grab message
    // raw_transaction = raw_transaction.trim().to_string();
    let raw_transaction = "02000000000101772251c4eb6c0fabdf689ca9703cdd107c6646b98f69d2fece5ef8e65112e06b0100000000feffffff01007083d05d060000225120ca1e131a2d01740a251d8bd0167bb032999b124c40ea23a1f87b9f5d713f97170140eb49c37a62ad556d55a42e560ef1a651ac32f5705ed06ce5185b63881eda4b269cb82338fb2b72ba7d35eb69659e440a0b4455d9c6cb6e5fd6de78c4ea0dd82cb4790800".to_string();


	let compressed_transaction = pretty_unwrap("Compress Transaction", serialize(&raw_transaction, &rpc));
	println!("compressed tx = {}", hex::encode(&compressed_transaction));
	// let compressed_transaction = [11, 21, 31, 40].to_vec();
	//STEP 2 Encrypt the transaction

	//TODO skiping

	//STEP 3 generate the jpeg and the probabilities file
	// /home/tom/app/dog.jpeg
	// let mut image_path = String::new(); //Create message string
    // print!("Please enter the path to your image: "); //Print to console
    // let _=stdout().flush(); //new line for console
    // stdin().read_line(&mut image_path).expect("Read Terminal Input"); //Grab message
    // image_path = image_path.trim().to_string();
    let image_path = "/home/tom/app/dog.jpeg".to_string();

	let img = ImageReader::open(image_path).expect("Open Image").decode().expect("Decode Image");
	let raw_val: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
	let index: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
	let buffer = File::create("prestego.jpeg").expect("Open Buffer");
	//92 is a popular compression rate(modifiable)
	let mut jpeg_encoder = JpegEncoder::new_with_quality(buffer, 92);

	jpeg_encoder.encode(img.as_bytes(), img.width(), img.height(), image::ColorType::Rgb8, Some(&|p| {
		let i = index.lock().unwrap().clone();
		//Only grab the CR CB DCT coefficents leave the Y untouched
		if (i % 3) as f32 != 0.0 {
			if i < 21 {
				println!("p = {}", p);
			}
			raw_val.lock().unwrap().push(p);
		}
		*index.lock().unwrap() += 1;
		p.round()
	})).expect("JPEG Encode");

	//Dispose of mutex
	let raw_val = raw_val.lock().unwrap().clone();

	//TODO parse this data from a file instead provided by gimp plugin
	let mut weights = Vec::new();
	let mut values = Vec::new();
	let mut i = 0;
	for val in raw_val {
		let mut weight = ((val%1.0)*100.0).abs() as u8;
			if weight >= 50 {
				weight -= 50;
			} else {
				weight = 50 - weight;
			}
			let lsb = (val.round() as i8 & (1 << 0) as i8) as u8;
			i += 1;
			values.push(lsb);
			weights.push(weight+1);
	}
	for i in 0..14 {
		println!("values = {}", values[i]);
	}


	//STEP 4 run the trelles stego on the list of LSBS with the probabilities
	let (stego_values, matrix) = pretty_unwrap("Stegoing LSBS", stego(&values, &weights, &compressed_transaction));
	for i in 0..14 {
		println!("stego = {}", stego_values[i]);
	}

	//STEP 5 decode jpeg file and modify the LSBS

	//TODO figure out how to just change LSBS of the DCTS after its been encoded.
	//Using original image and the same quality we get the same DCTs yet we can modify them after the stego
	let index: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
	let inde: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
	let buffer = File::create("stego.jpeg").expect("Open File");
	//92 is a popular compression rate(modifiable)
	let mut jpeg_encoder = JpegEncoder::new_with_quality(buffer, 92);

	jpeg_encoder.encode(img.as_bytes(), img.width(), img.height(), image::ColorType::Rgb8, Some(&|p| {
		let i = index.lock().unwrap().clone();
		let ind = inde.lock().unwrap().clone();
		*index.lock().unwrap() += 1;
		//Only grab the CR CB DCT coefficents leave the Y untouched
		if (i % 3) as f32 != 0.0 {
			*inde.lock().unwrap() += 1;
			if ind < 14 {
				println!("i < stego_values.len() = {}", ind < stego_values.len() as u32);
				println!("stego_values[{}] = {}", ind,  stego_values[ind as usize]);
				println!("values[{}] = {}", ind, values[ind as usize]);
			}
			if ind < stego_values.len() as u32 && stego_values[ind as usize] != values[ind as usize] {
				if p.round() < p {
					if ind < 14 {
						println!("p.c = {}", p.ceil());
					}
					return p.ceil()
				}
				if ind < 14 {
					println!("p.f = {}", p.floor());
				}
				return p.floor()
			}
			if ind < 14 {
				println!("p.r = {}", p.round());
			}
		}
		p.round()
	})).expect("JPEG Encode");


	//----DECODE

	//STEP 6 decode jpeg and read coeffients

	let file = File::open("stego.jpeg").expect("Read File");
    let mut decoder = jpeg::Decoder::new(BufReader::new(file));
	let index: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
	let cbb: Arc<Mutex<Vec<i16>>> = Arc::new(Mutex::new(Vec::new()));
	let crb: Arc<Mutex<Vec<i16>>> = Arc::new(Mutex::new(Vec::new()));
    decoder.decode(Some(&|coefficients| {
		let i = index.lock().unwrap().clone();
    	if (i % 3) as f32 == 1.0 {
    		cbb.lock().unwrap().extend(coefficients);
    	} else if (i % 3) as f32 == 2.0 {
    		crb.lock().unwrap().extend(coefficients);
    	}
    	*index.lock().unwrap() += 1;
    })).expect("failed to decode image");


	//STEP 7 compute the syndrom of the LSBS
	let cbb = cbb.lock().unwrap().clone();
	let crb = crb.lock().unwrap().clone();
	let mut image_values: Vec<u8> = Vec::new();

	
	for i in 0..cbb.len() {
		if i < 2 {
			println!("cr = {}", crb[i]);
		}
		image_values.push((cbb[i] as i8 & (1 << 0) as i8) as u8);
		image_values.push((crb[i] as i8 & (1 << 0) as i8) as u8);
	}
	for i in 0..7 {
		println!("cb = {}", cbb[i]);
	}
	for i in 0..7 {
		println!("cr = {}", crb[i]);
	}
	for i in 0..14 {
		println!("image_values = {}", image_values[i]);
	}
	for i in 0..14 {
		println!("stego = {}", stego_values[i]);
	}

	for i in 0..stego_values.len() {
		if stego_values[i] != image_values[i] {
			println!("i = {}", i);
			break
		}
	}
	//TODO: true cover length will be a multiple of the message, Current message length varies due to no encryption
	let image_values = image_values[0..(stego_values.len())].to_vec();
	assert_eq!(image_values == stego_values, true);

	//STEP 8 Decrypt via AES

	//TODO: skip this step

	//STEP 11 Compute syndrom

	let syndrom = matrix_multi(&image_values,  &matrix);
	// print!("bytes: ");
	// for i in 0..syndrom.len() {
	// 	print!("{}, ", syndrom[i]);
	// }
	// println!(";");
	let mut stego_result = Vec::new();
	for i in 0..(syndrom.len()/8) {
		let mut byte = "".to_string();
		for x in (i*8)..(i*8)+8 {
			byte += &syndrom[x].to_string();
		}
		println!("byte = {}", byte);
		stego_result.push(u8::from_str_radix(&byte, 2).unwrap());
	}
	assert_eq!(stego_result, compressed_transaction);


	//STEP 10 Decompress Transaction
	let bytes = Vec::from_hex(raw_transaction).expect("parse hex");

	//Deserialize Transaction
	let transaction = Transaction::deserialize(&bytes).expect("parse trans");

	pretty_unwrap("Deserialize Transaction", deserialize(&stego_result, &rpc, transaction));











	// //TEST COMPRESSION
	// let bytes = Vec::from_hex(raw_transaction).expect("parse hex");

	// //Deserialize Transaction
	// let transaction = Transaction::deserialize(&bytes).expect("parse trans");

	// pretty_unwrap("Deserialize Transaction", deserialize(&compressed_transaction, &rpc, transaction));























	// use image::codecs::jpeg::JpegEncoder;
	// use image::{Rgb, RgbImage, EncodableLayout};
	// use rand::Rng;

	// let a = 8;
	// let b = 8;
	// let mut img = RgbImage::new(a, b);
	// for x in 0..a {
	// 	for y in 0..b {
	// 		let mut rng = rand::thread_rng();
    // 		let g: u8 = rng.gen();
	// 		// println!("g = {}", g);
	// 		img.put_pixel(x, y, Rgb([200, g, 100]));
	// 	}
	// }
	// let mut buffer = File::create("/home/tom/app/test_dog.jpeg").expect("Open Image");
	// let mut jpeg_encoder = JpegEncoder::new_with_quality(buffer, 100);
	// jpeg_encoder.encode(img.as_bytes(), img.width(), img.height(), image::ColorType::Rgb8, None);
	

	// if cfg!(feature = "gui") {
	// 	#[cfg(feature="gui")]
	// 	pretty_unwrap("Run the GUI", gui());
	// } else {
	
	// println!("GUI unenabled");


	// pretty_unwrap("compressing image", compress_image("/home/a/Downloads/dog.jpeg".to_string()));


	//TRANSACTIONS
	//old tr
	//let txr = "02000000000101772251c4eb6c0fabdf689ca9703cdd107c6646b98f69d2fece5ef8e65112e06b0100000000feffffff01007083d05d060000225120ca1e131a2d01740a251d8bd0167bb032999b124c40ea23a1f87b9f5d713f97170140eb49c37a62ad556d55a42e560ef1a651ac32f5705ed06ce5185b63881eda4b269cb82338fb2b72ba7d35eb69659e440a0b4455d9c6cb6e5fd6de78c4ea0dd82cb4790800".to_string();
	//tr
	//let txr = "02000000000101f36f35b933bb1a136a0633585f2a70ab1877716bbcc46b0c04b4651fb134f68b0100000000ffffffff01a861000000000000160014ef28d520689aceb00b7c0264d97e505ece5fa2120140bbc1b94d6ba49f5f2c9c9ed7ef43f264ea60f6f107c2686d23f04d9cdc24608b190d43fc76f30afe4bf43cc45ca26ef109f7404cab5d5c761d874b3d757b355500000000".to_string();
	//p2pkh
	//let txr = "02000000010f7bb3af0aa10954a7fa555f78d058314e5869fa4c774bf4bdde41f153028c42010000006a473044022043145852fcdf1296680d764c161d6e24d1b4e4ad4ca1fd44f8fdd2ae7c8d2ecb022028e41a87ee6b39e38e57e8ee45f029678a8856b5139c78e77ffbbdd4d5f491fa0121038e0bfb625d7ef6182d653a7787f56f161a25c8e974186e9027b72bca8a569f1fffffffff01a861000000000000160014733154bc73b07fa0576b2a29a747753be1d0e8d800000000".to_string();
	//let txr = "01000000017afcd3403a2ee93dacdeda9802e42da8bb4e5e95223de3fc1ef2733f540786553f0000008b483045022100e6c21f3771ceb926cdeefa3784a0ddeba44089e731b536a560a7fda21d05bc31022061f8d8bf049966fe6a61ea8cfd0930d4e71229b39b1e48aa59c9471dd2233668014104350214d331d5947e8e9c6d937684385ff8e28d8055374704f26b8bdbd3c44d74ffd6af88865d350011c1255ddcdff416439e3a46b93b3fd463b906ff236beec0ffffffff02e6d50700000000001976a91453dce6052e05d0296ebc4c83bd24d0b108affc7988ac03d80000000000001976a91436bb1b3763fb824a23d84b163c9d0a060a79090388ac00000000".to_string();
	// pretty_unwrap("Compressing Transaction", compress_transaction(&txr, &rpc));





	//HUFFMAN STUFF
	// let mut bytes: Vec<u8> = Vec::new();
	// bytes.push(243);
	// bytes.push(243);
	// bytes.push(223);
	// bytes.push(243);
	// bytes.push(24);
	// bytes.push(23);
	// bytes.push(243);
	// bytes.push(24);
	// bytes.push(24);
	// bytes.push(24);
	// bytes.push(24);
	// bytes.push(24);
	// bytes.push(24);
	// bytes.push(24);
	// bytes.push(24);
	// bytes.push(32);
	// bytes.push(56);
	// bytes.push(30);
	// bytes.push(30);
	// bytes.push(30);
	// bytes.push(30);
	// bytes.push(30);
	// bytes.push(30);
	// bytes.push(30);
	// bytes.push(30);
	// bytes.push(34);
	// let huffman_table = pretty_unwrap("generating huffman", generate_huffman_table(&bytes));
	// for entry in &huffman_table {
	// 	println!("{} = {}", entry.byte, entry.encoding);
	// }
	// let (length, compressed_bytes) = pretty_unwrap("encoding with huffman", huffman_encode(&bytes, &huffman_table));
	// print!("encoding: ");
	// for byte in compressed_bytes {
	// 	print!("{}", format!("{:08b}", byte));
	// }
	// println!(";");

	// print!("table: ");
	// let serilized_table = pretty_unwrap("serlize huffman", serilize_huffman_table(&huffman_table, 100));
	// for byte in &serilized_table {
	// 	print!("{},", byte);
	// }
	// println!(";");
	// let (deserlized_table, ht_info) = pretty_unwrap("deserilize huffman", deserilize_huffman_table(&serilized_table));
	// for entry in &deserlized_table {
	// 	println!("{} = {}", entry.byte, entry.encoding);
	// }

	// assert_eq!(huffman_table, deserlized_table);
	// assert_eq!(ht_info, 100);


	// let mut bytes = Vec::new();
	// let mut file = File::open("/home/a/app/dog2.jpeg").expect("Could not find file");
	// let length = file.read_to_end(&mut bytes).unwrap();
	// for i in 0..length {
	// 	if bytes[i] == 255 && bytes[i+1] == 196 {
	// 		println!("DHT");
	// 	}
	// }
	// let image = ImageReader::open("/home/a/app/dog2.jpeg")?.decode()?.into_rgb8().into_raw();
	// let length = image.len();
	// for i in 0..length {
	// 	print!("{}, ", image[i]);
	// }
	



	//TEST COMPRESSION
	// use std::str::FromStr;
	// let public_key = "02350214d331d5947e8e9c6d937684385ff8e28d8055374704f26b8bdbd3c44d74".to_string();
	// let bpk: bitcoin::PublicKey = bitcoin::PublicKey::from_str(&public_key).expect("Parse Public Key");
	// let scpk1 = bitcoin::Script::new_p2pkh(&bpk.pubkey_hash());
	// let scpk2 = bitcoin::util::address::Address::p2pkh(&bpk, bitcoin::Network::Bitcoin).script_pubkey();
	// println!("scpk1 = {}", scpk1);
	// println!("scpk2 = {}", scpk2);
	// assert_eq!(scpk1, scpk2);
	// let bc = rpc.get_block_count().expect("Could Not Get Block Count");
 //    for y in 0..100000 {
 //        let i = bc - y;
 //        println!("B-----------------------------------------------------I = {}", i);
 //        let bh = rpc.get_block_hash(i).expect("Could Not Get Block Hash");
 //        let txs = rpc.get_block_info(&bh).expect("Could Not Get Block Info").tx;
 //        for x in 0..txs.len() {
 //            println!("T-----------------------------------------------B = {} X = {}", i, x);
 //            if x > 0 {
 //            	let tx = txs[x];
 //            	println!("tx = {}", tx);
 //            	let transaction = rpc.get_raw_transaction_hex(&tx, None).expect("Could Not Find Transaction");
 //            	let ctx = pretty_unwrap("Compressing Transaction", compress_transaction(&transaction, &rpc));
 //            	println!("tranlen = {}, ctxlen = {}, diff = {}", transaction.len(), ctx.len(), transaction.len()-ctx.len());
 //            }
 //        }
 //    }
}


// createrawtransaction '[{"txid":"8bf634b11f65b4040c6bc4bc6b717718ab702a5f5833066a131abb33b9356ff3","vout":1}]' '[{"bc1qau5d2grgnt8tqzmuqfjdjljstm89lgsjuxqgu5": 0.00025}]'