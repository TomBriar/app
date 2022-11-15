use crate::error::Error;
use std::{cmp, mem};
use std::collections::{HashMap, HashSet, BinaryHeap};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entry {
	pub byte: u8,
	pub encoding: String
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Node {
	Leaf {
		byte: u8,
	},
	Internal {
		left: Box<Self>,
		right: Box<Self>,
	},
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Weighted<T> {
	node: T,
	weight: u32,
}
// Sort by weight, then by node. Reverse the order because we're going
// to use this with a `BinaryHeap`, which inexplicably is a max-heap
// even though you almost always want a min-heap.
impl<T: cmp::Ord + Eq> cmp::Ord for Weighted<T> {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.weight
			.cmp(&other.weight)
			.then(self.node.cmp(&other.node))
			.reverse()
	}
}
impl<T: cmp::Ord + Eq> cmp::PartialOrd for Weighted<T> {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T> Weighted<T> {
	/// Construct a new `Weighted` with a given weight
	fn new(node: T, weight: u32) -> Weighted<T> { Weighted { node, weight } }
}

pub fn generate_huffman_table(bytes: &[u8]) -> HashSet<Entry> {
	// Early-return on empty table
	if bytes.is_empty() {
		return HashSet::new();
	}

	// Count occurrence of each bye
	let mut weights = HashMap::with_capacity(cmp::min(256, bytes.len()));
	for byte in bytes {
		*weights.entry(*byte).or_insert(0) += 1;
	}
	let weights = weights; // prevent further mutation

	// Put them into a heap ordered by height
	let mut heap: BinaryHeap<Weighted<Node>> = BinaryHeap::new();
	for (byte, weight) in weights {
		heap.push(Weighted::new(Node::Leaf { byte }, weight));
	}

	// Combine highest-weighted nodes until only one is left
	assert!(!heap.is_empty());
	while heap.len() > 1 {
		// unwraps are safe since our loop condition guarantees >= 2 elements
		let mut first = heap.pop().unwrap();
		let mut second = heap.pop().unwrap();

		// Make `first` always >= `second`. (Remembering that we're in
                // backwards-heap-land where ">=" means "<="
		if first < second {
			mem::swap(&mut first, &mut second);
		}
		// Combine and reinsert
		heap.push(Weighted::new(
			Node::Internal { left: Box::new(first.node), right: Box::new(second.node) },
			first.weight + second.weight,
		));
	}
	assert_eq!(heap.len(), 1);
	let tree = heap.pop().unwrap().node;

        // Iterate through tree to define encoding
	let mut huffman_table = HashSet::with_capacity(cmp::min(256, bytes.len()));
	let mut node_stack = vec![(tree, "".to_string())];
	loop {
		match node_stack.pop() {
			None => break,
			Some((Node::Leaf { byte }, current_path)) => {
				huffman_table.insert(Entry{
					byte: byte,
					encoding: current_path
				});
			},
			Some((Node::Internal { left, right }, current_path)) => {
				node_stack.push((*left, current_path.clone() + "0"));
				node_stack.push((*right, current_path.clone() + "1"));
			},
		}
	}

	huffman_table
}

pub fn serilize_huffman_table(huffman_table: &[Entry], ht_info: u8) -> Result<Vec<u8>, Error> {
	let mut sorted_huffman_table = huffman_table.to_vec();
	sorted_huffman_table.sort_unstable_by_key(|e| e.encoding.len());
	let mut huffman_symbol_count: Vec<u8> = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].to_vec();
	let mut symbols: Vec<u8> = Vec::new();
	for entry in huffman_table {
		huffman_symbol_count[entry.encoding.len()] += 1;
		symbols.push(entry.byte);
	}

	let huffman_length: u16 = 2+2+1+huffman_symbol_count.len() as u16 + symbols.len() as u16;

	let mut result: Vec<u8> = Vec::new();
	result.push(0xff);
	result.push(0xc4);
	result.extend(huffman_length.to_be_bytes());
	result.push(ht_info);
	result.extend(huffman_symbol_count);
	result.extend(symbols);
	Ok(result)
}

pub fn huffman_encode(bytes: &[u8], huffman_table: &[Entry]) -> Result<(usize, Vec<u8>), Error> {
	let mut encoded = String::new();
	for byte in bytes {
		for entry in huffman_table {
			if &entry.byte == byte {
				encoded += &entry.encoding;
				break
			}
		}
	}
	let length = encoded.len();
	let mut result = Vec::new();
	let mut index = 0;
	loop {
		println!("index = {}, length = {}", index, length);
		if index+8 <= length {
			println!("push");
			let byte_string = &encoded[index..index+8];
			
			result.push(u8::from_str_radix(byte_string, 2).unwrap())
		}
		index += 8;
		if index+8 > length {
			println!("over fill");
			let mut byte_string = encoded[index..length].to_string();
			for _ in byte_string.len()..8 {
				byte_string += "0";
			}
			result.push(u8::from_str_radix(&byte_string, 2).unwrap());
			break
		}
	}
	Ok((length, result))
}


pub fn deserilize_huffman_table(serilized_huffman_table: &[u8]) -> Result<(Vec<Entry>, u8), Error> {
	if serilized_huffman_table[0] != 0xff || serilized_huffman_table[1] != 0xc4 {
		return Err(Error::UnParsableHuffmanTable)
	}
	let ht_info = &serilized_huffman_table[4];
	let symbol_count = &serilized_huffman_table[5..5+16].to_vec();
	let symbols = &serilized_huffman_table[5+16..serilized_huffman_table.len()];
	let mut huffman_table = Vec::new();
	let mut encoding = 0;
	let mut index = 0;
        for (i, symbol) in symbol_count.iter().copied().enumerate() {
		for _ in 0..symbol {
			let mut int = format!("{:b}", encoding);
			for _ in int.len()..i {
				int = "0".to_string()+&int;
			}
			println!("byte = {}, encode = {:b}", symbols[index], encoding);
			huffman_table.push(Entry{
				byte: symbols[index],
				encoding: int
			});
			encoding += 1;
			index += 1;
		}
		if symbol > 0 {
			encoding += 1;
			encoding *= 2;
		}
	}
	Ok((huffman_table, *ht_info))
}

#[allow(unused_variables)]
pub fn huffman_decode(huffman_table: &[Entry], data: &[u8]) -> Result<Vec<u8>, Error> {
	Ok([0].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! set {
        ($($ch:expr => $s:expr),*) => (
            {
                let mut set = HashSet::new();
                $(set.insert(Entry { byte: $ch, encoding: $s.into() });)*
                set
            }
        );
    }

    #[test]
    fn huffman_basic() {
        assert_eq!(generate_huffman_table(&[]), HashSet::new());
        assert_eq!(
            generate_huffman_table(b"A"),
            set!(b'A' => ""),
        );

        assert_eq!(
            generate_huffman_table(b"ABCD"),
            set!(b'A' => "00", b'B' => "01", b'C' => "10", b'D' => "11"),
        );

        assert_eq!(
            generate_huffman_table(b"AACD"),
            set!(b'A' => "0", b'C' => "10", b'D' => "11"),
        );

        assert_eq!(
            generate_huffman_table(b"ACCD"),
            set!(b'C' => "0", b'A' => "10", b'D' => "11"),
        );
        assert_eq!(
            generate_huffman_table(b"DACC"),
            set!(b'C' => "0", b'A' => "10", b'D' => "11"),
        );

    }
}

