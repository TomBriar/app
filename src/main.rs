extern crate app;
extern crate bitcoincore_rpc;

use app::transaction::serialize;
use app::transaction::deserialize;
use app::error::Error;
use app::stego::{stego, matrix_multi};
use app::tests::run_tests;
use home;



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
	run_tests();
	//STEP 0 parse bitcoin.conf
	let mut rpcport = String::new();
	let mut rpcuser = String::new();
	let mut rpcpass = String::new();
	let dot_bitcoin = home::home_dir().unwrap().to_str().unwrap().to_owned()+"/.bitcoin";
	println!("dot = {}", dot_bitcoin);
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

    let bytes = Vec::from_hex(raw_transaction).expect("Hex to Bytes");
	let transaction = Transaction::deserialize(&bytes).expect("Deserialize Raw Transaction");

	let compressed_transaction = pretty_unwrap("Compress Transaction", serialize(&transaction, &rpc));
	println!("compressed tx = {}", hex::encode(&compressed_transaction));
	//STEP 2 Encrypt the transaction

	//TODO skiping

	//STEP 3 generate the jpeg and the probabilities file
	// /home/tom/app/dog.jpeg
	// let mut image_path = String::new(); //Create message string
    // print!("Please enter the path to your image: "); //Print to console
    // let _=stdout().flush(); //new line for console
    // stdin().read_line(&mut image_path).expect("Read Terminal Input"); //Grab message
    // image_path = image_path.trim().to_string();
    let image_path = home::home_dir().unwrap().to_str().unwrap().to_owned()+"/app/dog.jpeg";

	let img = ImageReader::open(image_path).expect("Open Image").decode().expect("Decode Image");
	let raw_val: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
	let index: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
	let buffer = File::create("prestego.jpeg").expect("Open Buffer");
	//92 is a popular compression rate(modifiable)
	let mut jpeg_encoder = JpegEncoder::new_with_quality(buffer, 82);

	jpeg_encoder.encode(img.as_bytes(), img.width(), img.height(), image::ColorType::Rgb8, Some(&|p| {
		let i = index.lock().unwrap().clone();
		//Only grab the CR CB DCT coefficents leave the Y untouched
		if (i % 3) as f32 != 0.0 {
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

	//STEP 4 run the trelles stego on the list of LSBS with the probabilities
	let (stego_values, matrix) = pretty_unwrap("Stegoing LSBS", stego(&values, &weights, &compressed_transaction));

	//STEP 5 decode jpeg file and modify the LSBS

	//TODO figure out how to just change LSBS of the DCTS after its been encoded.
	//Using original image and the same quality we get the same DCTs yet we can modify them after the stego
	let index: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
	let inde: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
	let buffer = File::create("stego.jpeg").expect("Open File");
	//92 is a popular compression rate(modifiable)
	let mut jpeg_encoder = JpegEncoder::new_with_quality(buffer, 82);

	jpeg_encoder.encode(img.as_bytes(), img.width(), img.height(), image::ColorType::Rgb8, Some(&|p| {
		let i = index.lock().unwrap().clone();
		let ind = inde.lock().unwrap().clone();
		*index.lock().unwrap() += 1;
		//Only grab the CR CB DCT coefficents leave the Y untouched
		if (i % 3) as f32 != 0.0 {
			*inde.lock().unwrap() += 1;
			if ind < stego_values.len() as u32 && stego_values[ind as usize] != values[ind as usize] {
				if p.round() < p {
					return p.ceil()
				}
				return p.floor()
			}
		}
		p.round()
	})).expect("JPEG Encode");

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
		image_values.push((cbb[i] as i8 & (1 << 0) as i8) as u8);
		image_values.push((crb[i] as i8 & (1 << 0) as i8) as u8);
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
		stego_result.push(u8::from_str_radix(&byte, 2).unwrap());
	}
	assert_eq!(stego_result, compressed_transaction);


	//STEP 10 Decompress Transaction
	let trans = pretty_unwrap("Deserialize Transaction", deserialize(&stego_result, &rpc));

	assert_eq!(trans, transaction);

}


// createrawtransaction '[{"txid":"8bf634b11f65b4040c6bc4bc6b717718ab702a5f5833066a131abb33b9356ff3","vout":1}]' '[{"bc1qau5d2grgnt8tqzmuqfjdjljstm89lgsjuxqgu5": 0.00025}]'