use std::str::FromStr;
use std::fs::File;
use std::io::Read;
use bitcoincore_rpc::{RpcApi, Auth, Client};
use hex::FromHex;
use bitcoin::Transaction;
use bitcoin::psbt::serialize::Deserialize;

use crate::stego::{stego, matrix_multi};
use crate::transaction::{serialize, deserialize};

fn test_exaustive_compression_roundtrip(rpc: &bitcoincore_rpc::Client) {
	let bc = rpc.get_block_count().expect("Could Not Get Block Count");
    for y in 0..100000 {
        let i = bc - y;
        println!("B-----------------------------------------------------I = {}", i);
        let bh = rpc.get_block_hash(i).expect("Could Not Get Block Hash");
        let txs = rpc.get_block_info(&bh).expect("Could Not Get Block Info").tx;
        for x in 0..txs.len() {
            println!("T-----------------------------------------------B = {} X = {}", i, x);
            if x > 0 {
            	let tx = txs[x];	
            	println!("tx = {}", tx);
            	let raw_transaction = rpc.get_raw_transaction_hex(&tx, None).expect("Could Not Find Transaction");
				let bytes = Vec::from_hex(tx).expect("Hex to Bytes");
				let transaction = Transaction::deserialize(&bytes).expect("Deserialize Raw Transaction");
            	let serialized_transaction = serialize(&transaction, &rpc).expect("Serilize Transaction");
            	let trans = deserialize(&serialized_transaction, &rpc).expect("Deserialize Transaction");
            	println!("transaction == trans = {}", transaction == trans);
				if transaction != trans {
					println!("transaction.input == trans.input = {}", transaction.input == trans.input);
					if transaction.input != trans.input {
						for a in 0..transaction.input.len() {
							println!("transaction.input[{}].previous_output == trans.input[{}].previous_output = {}", a, a, transaction.input[a].previous_output == trans.input[a].previous_output);
							println!("transaction.input[{}].script_sig == trans.input[{}].script_sig = {}", a, a, transaction.input[a].script_sig == trans.input[a].script_sig);
							if transaction.input[a].script_sig != trans.input[a].script_sig {
								println!("transaction.input[{}].script_sig = {}", a, transaction.input[a].script_sig);
								println!("trans.input[{}].script_sig = {}", a, trans.input[a].script_sig);
							}
							println!("transaction.input[{}].sequence == trans.input[{}].sequence = {}", a, a, transaction.input[a].sequence == trans.input[a].sequence);
							println!("transaction.input[{}].witness == trans.input[{}].witness = {}", a, a, transaction.input[a].witness == trans.input[a].witness);
							if transaction.input[a].witness != trans.input[a].witness {
								println!("hex::encode(transaction.input[{}].witness.to_vec()[b]): ", a);
								for b in 0..transaction.input[a].witness.to_vec().len() {
									println!("	hex::encode(transaction.input[{}].witness.to_vec()[{}]) = {}", a, b, hex::encode(&transaction.input[a].witness.to_vec()[b]));
								}
								println!("hex::encode(trans.input[{}].witness.to_vec()[b]): ", a);
								for b in 0..trans.input[a].witness.to_vec().len() {
									println!("	hex::encode(trans.input[{}].witness.to_vec()[{}]) = {}", a, b, hex::encode(&trans.input[a].witness.to_vec()[b]));
								}
							}
						}
					}
					println!("transaction.output == trans.output = {}", transaction.output == trans.output);
					println!("transaction.version == trans.version = {}", transaction.version == trans.version);
					println!("transaction.lock_time == trans.lock_time = {}", transaction.lock_time == trans.lock_time);
					panic!("Could not compress transaction");
				}
            }
        }
    }
}

fn test_compression_roundtrip(rpc: &bitcoincore_rpc::Client, debug: bool) {
    let raw_transaction = "02000000000101772251c4eb6c0fabdf689ca9703cdd107c6646b98f69d2fece5ef8e65112e06b0100000000feffffff01007083d05d060000225120ca1e131a2d01740a251d8bd0167bb032999b124c40ea23a1f87b9f5d713f97170140eb49c37a62ad556d55a42e560ef1a651ac32f5705ed06ce5185b63881eda4b269cb82338fb2b72ba7d35eb69659e440a0b4455d9c6cb6e5fd6de78c4ea0dd82cb4790800".to_string();
	let bytes = Vec::from_hex(raw_transaction).expect("Hex to Bytes");
	let transaction = Transaction::deserialize(&bytes).expect("Deserialize Raw Transaction");
	let serialized_transaction = serialize(&transaction, &rpc).expect("Serilize Transaction");
	let trans = deserialize(&serialized_transaction, &rpc).expect("Deserialize Transaction");

	if debug {
		println!("transaction == trans = {}", transaction == trans);
		if transaction != trans {
			println!("transaction.input == trans.input = {}", transaction.input == trans.input);
			if transaction.input != trans.input {
				for a in 0..transaction.input.len() {
					println!("transaction.input[{}].previous_output == trans.input[{}].previous_output = {}", a, a, transaction.input[a].previous_output == trans.input[a].previous_output);
					println!("transaction.input[{}].script_sig == trans.input[{}].script_sig = {}", a, a, transaction.input[a].script_sig == trans.input[a].script_sig);
					if transaction.input[a].script_sig != trans.input[a].script_sig {
						println!("transaction.input[{}].script_sig = {}", a, transaction.input[a].script_sig);
						println!("trans.input[{}].script_sig = {}", a, trans.input[a].script_sig);
					}
					println!("transaction.input[{}].sequence == trans.input[{}].sequence = {}", a, a, transaction.input[a].sequence == trans.input[a].sequence);
					println!("transaction.input[{}].witness == trans.input[{}].witness = {}", a, a, transaction.input[a].witness == trans.input[a].witness);
					if transaction.input[a].witness != trans.input[a].witness {
						println!("hex::encode(transaction.input[{}].witness.to_vec()[b]): ", a);
						for b in 0..transaction.input[a].witness.to_vec().len() {
							println!("	hex::encode(transaction.input[{}].witness.to_vec()[{}]) = {}", a, b, hex::encode(&transaction.input[a].witness.to_vec()[b]));
						}
						println!("hex::encode(trans.input[{}].witness.to_vec()[b]): ", a);
						for b in 0..trans.input[a].witness.to_vec().len() {
							println!("	hex::encode(trans.input[{}].witness.to_vec()[{}]) = {}", a, b, hex::encode(&trans.input[a].witness.to_vec()[b]));
						}
					}
				}
			}
			println!("transaction.output == trans.output = {}", transaction.output == trans.output);
			println!("transaction.version == trans.version = {}", transaction.version == trans.version);
			println!("transaction.lock_time == trans.lock_time = {}", transaction.lock_time == trans.lock_time);
		}
		panic!("Could not round trip compression");
	} else {
		assert_eq!(transaction, trans);
	}
}

fn test_stego(rpc: &bitcoincore_rpc::Client, debug: bool) {
	let values = [0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1].to_vec();
	let weights = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0].to_vec();
	let message = "t".to_string().as_bytes().to_vec();
	let (stego_values, matrix) = stego(&values, &weights, &message).expect("stego");
	let syndrom = matrix_multi(&stego_values,  &matrix);
	let mut stego_result = Vec::new();
	for i in 0..(syndrom.len()/8) {
		let mut byte = "".to_string();
		for x in (i*8)..(i*8)+8 {
			byte += &syndrom[x].to_string();
		}
		stego_result.push(u8::from_str_radix(&byte, 2).unwrap());
	}
	if debug {
		panic!("NO DEBUG");
	} else {
		assert_eq!(message, stego_result)
	}
}

pub fn run_tests() {
	let mut rpcport = String::new();
	let mut rpcuser = String::new();
	let mut rpcpass = String::new();
	let dot_bitcoin = home::home_dir().unwrap().to_str().unwrap().to_owned()+"/.bitcoin";
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

    let debug = false;

    test_compression_roundtrip(&rpc, debug);
    test_stego(&rpc, debug);
}