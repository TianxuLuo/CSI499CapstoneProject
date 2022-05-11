fn encrypt(original_data: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
	let mut result: Vec<u8> = Vec::new();

	let original_data_len = original_data.len();
	let key_len = key.len();
	let mut encrypted_data_len = (original_data_len + key_len - 1) / key_len * key_len;
	while encrypted_data_len < original_data_len + 4 {
		encrypted_data_len += key_len;
	}

	let mut data = original_data.clone();
	for _ in 0..encrypted_data_len - original_data_len - 4 {
		data.push(0);
	}
	data.push(((original_data_len as u64) & 0xff) as u8);
	data.push((((original_data_len as u64) >> 8) & 0xff) as u8);
	data.push((((original_data_len as u64) >> 16) & 0xff) as u8);
	data.push((((original_data_len as u64) >> 24) & 0xff) as u8);

	let mut index_of_data = 0;
	let mut index_of_key = 0;
	let times = encrypted_data_len / key_len;
	for _ in 0..times {
		for _ in 0..key_len {
			result.insert(0, data[index_of_data] ^ key[index_of_key]);
			index_of_data += 1;
			index_of_key = (index_of_key + 1) % key_len;
		}
	}

	result
}

fn decrypt(encrypted_data: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
	let mut result: Vec<u8> = Vec::new();

	let encrypted_data_len = encrypted_data.len();
	let key_len = key.len();

	let mut data = encrypted_data.clone();
	data.reverse();

	let mut original_data: Vec<u8> = Vec::new();
	let mut index_of_data = 0;
	let mut index_of_key = 0;
	let times = encrypted_data_len / key_len;
	for _ in 0..times {
		for _ in 0..key_len {
			original_data.push(data[index_of_data] ^ key[index_of_key]);
			index_of_data += 1;
			index_of_key = (index_of_key + 1) % key_len;
		}
	}

	let value3 = original_data[original_data.len() - 1] as u8;
	let value2 = original_data[original_data.len() - 2] as u8;
	let value1 = original_data[original_data.len() - 3] as u8;
	let value0 = original_data[original_data.len() - 4] as u8;
	let original_data_len = ((value3 as u32) << 24)
		+ ((value2 as u32) << 16)
		+ ((value1 as u32) << 8)
		+ (value0 as u32);

	for i in 0..(original_data_len as usize) {
		result.push(original_data[i]);
	}

	result
}
