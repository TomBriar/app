use std::f64::INFINITY;
// use std::sync::{Arc, Mutex};
// use std::io::{stdout, stdin};
use crate::error::Error;

pub fn matrix_multi(x: &Vec<u8>, ch: &Vec<Vec<u8>>) -> Vec<u8> { //multiplys a vector of length equal to that of the cover object against the extended matrix. The result is a syndrom the length of the message.
		let mut s = Vec::new();
		for _ in 0..ch.len() {
			s.push(0);
		}
		for i in 0..ch.len() {
			for ii in 0..ch[0].len() {
				// if (i == (ch.len()-1)) {
				// 	println!("ch[{}][{}] = {}, x[{}] = {}", i, ii, ch[i][ii], ii, x[ii]);
				// }
				s[i] = (s[i]+((x[ii] as u8*ch[i][ii])%2))%2;
			}
		}
		for i in 0..ch.len() {
			s[i] = s[i]%2;
		}
		return s
	}


pub fn stego(cover_object_untrimmed: &Vec<u8>, cover_weights: &Vec<u8>, message: &Vec<u8>) -> Result<(Vec<u8>, Vec<Vec<u8>>), Error> {

	let mut message_bits = Vec::new();
	for byte in message { 
		let mut binary = format!("{byte:b}");
		for _ in 0..(8-binary.len()) { 
			let filler: String = String::from("0");
			binary = filler+&binary
		}
		for i in binary.chars() {
			message_bits.push(i.to_string().parse::<u8>().unwrap());
		}
	}
	let message = message_bits;
	// print!("message: ");
	// for i in 0..message.len() {
	// 	print!("{}, ", message[i]);
	// }
	// println!(";");

	let cover_object: Vec<u8> = cover_object_untrimmed[0..cover_object_untrimmed.len()-(cover_object_untrimmed.len()%message.len())].to_vec();
	assert!(cover_object.len()%message.len() == 0);
	// println!("cover_object.len() = {}", cover_object.len());
	// println!("message.len() = {}", message.len());
	let sub_width = cover_object.len()/message.len(); //rate of the encoding or the width of the sub matrix H
    let sub_height: usize = 4; //performance parameter
    let h = 2_u8.pow(sub_height as u32); //2^h
    let mut sub_h: Vec<Vec<u8>> = Vec::new(); //create the sub_h or h_hat vector
    for i in 0..sub_height {
    	sub_h.push(Vec::new());
    	for x in 0..sub_width {
    		if ((x+i)%sub_height) == 0 { //randomly push a zero or one
    			sub_h[i].push(1); 
    		} else {
    			sub_h[i].push(0);
    		}
    	} 
    }
    println!("sub_h:");
    for i in 0..sub_h.len() {
    	for x in 0..sub_h[i].len() {
    		print!("{}, ", sub_h[i][x]);
    	}
    	println!(".");
    }
    println!(";");

    let mut sub_ch: Vec<Vec<u8>> = Vec::new(); //create the column oriented sub_h or h_hat
    for i in 0..sub_width {
    	sub_ch.push(Vec::new());
    	for ii in 0..sub_height {
    		sub_ch[i].push(sub_h[ii][i]);
    	}
    }

    let mut ph_hat: Vec<Vec<u8>> = Vec::new(); //A vector of vectors, the first of which contains the int format for each column of sub_h or h_hat, the remaining vectors contain the trimed columns based on the extended H. 
    for i in 0..sub_height { //The number of trimed column blocks
    	ph_hat.push(Vec::new());
    	for ii in 0..sub_width {
    		ph_hat[i].push(0);
    		for iii in 0..(sub_height-i) { //the number of bits per trimmed column
    			ph_hat[i][ii] += sub_ch[ii][iii]*2_u8.pow(iii as u32); //binary to int
    		}
    	}
    }
 
    let ext_height = message.len(); //extended matrix H height
    let ext_width = cover_object.len(); //extended matrix W width
    let b = ext_width/sub_width; //Number of copies of sub_h or h_hat in the extended matrix. Includes trimmed blocks.


    let mut ext_h: Vec<Vec<u8>> = Vec::new(); //extended matrix
    for i in 0..(ext_height) {
    	ext_h.push(Vec::new());
    	for _ in 0..ext_width {
    		ext_h[i].push(0);
    	}
    }

    let mut ext_ch: Vec<Vec<u8>> = Vec::new(); //extended matrix column oriented
    for i in 0..(ext_width) {
    	ext_ch.push(Vec::new());
    	for _ in 0..ext_height {
    		ext_ch[i].push(0);
    	}
    }

    let mut row = 0;
    let mut column = 0;
    'B: for _ in 0..(ext_width/sub_width) { //Builds the extended matrix
    	'H: for ii in 0..sub_height {
    		for iii in 0..sub_width {
    			if row+ii >= ext_height {
    				break 'H
    			}
    			if column+iii >= ext_width {
    				break 'B
    			}
    			ext_h[row+ii][column+iii] = sub_h[ii][iii];
    		}
    	}
    	row += 1;
		column = column+sub_width;
    }
    
    for i in 0..ext_h[0].len() { //Builds the column oriented extended matrix
    	for ii in 0..ext_h.len() {
    		ext_ch[i][ii] = ext_h[ii][i];
    	}
    }

	let mut path: Vec<Vec<u8>> = Vec::new(); //path vector of vectors contains a vector of each state for each column.
	for i in 0..cover_object.len() {
		path.push(Vec::new());
		for _ in 0..h {
			path[i].push(0);
		}
	}
	let mut wght: Vec<f64> = Vec::new(); //contains the cost per path
	wght.push(0.0);
	for _ in 1..h {
		wght.push(INFINITY);
	}
	let mut y: Vec<u8> = Vec::new(); //stego cover object
	for _ in 0..cover_object.len() {
		y.push(0);
	}
	let mut indx = 0;
	let mut indm = 0;

	//FORWARD RUN
	for _ in 1..((b+1) as usize) { //For each copy of sub_h in ext_h
		for j in 0..((sub_width) as usize) { //for each column
			let mut newwght: Vec<f64> = Vec::new();
			for _ in 0..h {
				newwght.push(INFINITY);
			}
			for k in 0..(h as usize) { //for each state 
				let mut phindex = 0; 
				if (indm+sub_height) > b { //Decides if the current column is a trimed version of sub_h or h_hat
					phindex = (indm+sub_height)-b;
				}
				let w0 = wght[k] + ((cover_object[indx]*cover_weights[indx]) as f64); //weight of not adding the current column of sub_h or h_hat
				let w1 = wght[((k as u8)^ph_hat[phindex][(j%sub_width) as usize]) as usize] + ((((1+cover_object[indx])%2)*cover_weights[indx]) as f64); //weight of adding the current column of sub_h or h_hat
				path[indx][k] = if w1 < w0 { //recordes the available paths for this state
					1
				} else {
					0
				};
				// println!("j = {}, w0 = {}, w1 = {}, p[{}][{}] = {}", j, w0, w1, indx, k, path[indx][k]);
				newwght[k] = w0.min(w1); //decides if adding or not addingh the column of h_hat was cheeper
			}
			indx += 1;
			wght = newwght;
		}
		for j in 0..h/2 {
			// println!("{}, {}", indm, message[indm]);
			wght[j as usize] = wght[((2*j) + message[indm]) as usize]; // squashes the weights by half taking either the even node or the odd node based on the message bit
		}
		for j in h/2..h {
			wght[j as usize] = INFINITY; //zeros out the second half after the squash
		}
		indm += 1;
	}
	let embeding_cost = wght[0]; 
	println!("embeding cost = {}", embeding_cost);

	//BACKWARDS RUN
	let mut state: u8 = message[(indm-1) as usize]; //current state of the trellis //message[(indm-1) as usize]
	for _ie in 1..((b+1) as usize) { //for each copy of sub_h or h_hat
		indm -= 1;
		// let _i = b-ie; // To go backwards
		for je in 1..((sub_width+1) as usize) { //for each column
			indx -= 1;
			let j = sub_width-je; // To go backwards
			y[indx] = path[indx][state as usize] as u8; //set the stego object bit for this state
			let mut phindex = 0;
			if (indm+sub_height) > b { //decides if we need to use a trimed copy of h_hat or sub_h
				phindex = (indm+sub_height)-b; 
			}
			state = state^((y[indx] as u8*ph_hat[phindex][(j%sub_width) as usize])); //updates the state based on cheepest choice
		}
		if indm == 0 {
			break
		}
		state = (2*state + message[indm-1 as usize]) % h; //updates the state to account for the pruning
	}

	let syndrom = matrix_multi(&y,  &ext_h);
	assert!(syndrom == *message); // Assert virtubi succeded
	return Ok((y, ext_h))
}