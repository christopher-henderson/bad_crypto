const NUM_SBOXES: usize = 5;
const SBOX_LENGTH: usize = 256;
const PBOX_LEGNTH: usize = 32;

static SBOXES: [[u8; SBOX_LENGTH]; NUM_SBOXES] = [
	[19,183,197,34,33,140,178,87,230,71,187,64,105,24,231,110,58,175,145,132,45,242,198,72,252,141,36,165,59,88,63,207,215,51,203,13,37,247,79,209,222,159,143,81,43,155,57,227,244,114,17,76,156,30,78,137,39,217,69,27,213,220,188,193,184,128,50,158,56,250,161,20,55,253,223,113,12,98,144,168,7,179,246,166,171,60,48,6,126,115,160,211,176,95,80,91,130,194,4,15,3,134,173,218,53,120,49,216,255,118,103,127,149,124,38,189,21,162,92,249,151,83,52,236,44,85,32,97,70,125,42,74,142,75,1,102,169,163,119,101,14,66,117,206,225,116,152,121,108,93,11,153,185,251,191,202,122,164,8,129,167,228,136,104,62,31,65,201,84,135,123,73,90,174,239,199,190,214,192,109,180,133,22,233,170,67,240,100,196,146,54,107,94,224,35,157,29,221,232,2,237,77,195,248,150,186,210,238,181,154,148,245,177,205,68,234,47,106,243,40,182,111,10,219,89,23,241,82,226,200,208,16,229,86,18,99,28,147,46,254,0,212,235,25,26,131,61,5,172,138,9,41,112,204,96,139],
	[125,31,231,40,181,133,100,132,206,151,154,35,169,71,8,216,115,123,65,58,230,116,237,59,32,110,152,119,17,184,180,205,255,121,21,68,91,55,165,223,7,192,249,72,229,190,60,126,108,220,131,111,70,174,209,51,196,101,10,25,247,127,33,137,129,30,107,176,38,235,54,96,185,89,24,167,252,37,238,210,36,246,69,41,19,251,128,242,34,23,5,82,61,221,224,236,157,94,93,63,64,142,198,0,234,240,46,194,53,188,201,11,113,29,9,74,189,99,248,22,79,218,141,155,153,143,124,159,87,208,135,3,15,241,118,215,228,213,214,173,84,92,73,45,254,161,160,144,197,76,88,175,39,245,80,98,232,26,117,16,67,85,42,122,225,199,14,226,27,81,146,139,28,182,178,12,109,212,211,203,202,148,97,105,171,195,75,179,145,193,44,103,183,50,170,49,57,6,162,138,168,43,1,156,52,136,13,134,66,47,56,219,102,244,104,130,147,158,95,177,250,163,243,112,200,239,114,191,164,20,140,187,106,149,86,172,166,18,222,186,77,227,207,204,90,4,2,48,62,150,83,217,233,120,78,253],
	[90,53,65,72,87,144,197,86,59,135,247,32,116,38,89,3,14,84,188,113,126,18,224,183,60,220,148,210,43,171,214,137,154,252,133,217,71,146,50,123,2,12,186,110,66,78,168,163,226,68,193,115,201,225,91,240,139,4,158,179,16,175,37,239,51,1,5,196,120,191,207,97,64,187,92,149,227,230,119,162,174,249,26,73,23,131,124,79,128,209,155,7,15,52,152,180,134,45,105,6,107,218,243,130,173,194,125,157,167,159,172,245,29,111,145,82,62,203,215,141,204,88,85,181,237,160,106,36,63,76,195,41,22,189,122,13,212,246,40,223,205,190,34,185,178,9,31,58,109,17,20,234,96,165,200,241,28,83,77,221,211,222,44,206,94,61,33,176,138,112,232,129,104,229,102,95,10,70,161,47,219,244,8,142,156,151,30,98,169,75,42,27,231,150,56,67,164,202,242,170,117,254,93,199,213,39,127,103,192,114,153,250,11,121,253,166,19,118,80,143,54,184,74,100,216,208,182,25,140,81,255,55,238,177,248,69,101,251,57,108,48,35,0,236,235,233,147,99,24,21,228,198,49,132,136,46],
	[223,62,236,240,19,74,227,77,143,12,49,57,85,10,146,8,17,245,98,55,123,151,215,64,73,144,208,70,176,157,93,178,179,61,231,127,192,96,125,13,104,214,238,92,65,34,79,99,78,117,9,242,212,185,254,118,186,54,235,44,201,160,173,66,145,20,164,182,18,161,206,251,184,100,195,67,181,194,187,103,147,38,177,121,158,188,43,31,255,225,210,68,27,148,243,196,247,69,239,130,163,30,190,88,171,42,95,32,51,203,139,165,81,207,204,48,230,87,233,136,47,126,175,183,6,133,109,249,172,46,237,197,25,222,50,111,224,84,22,193,41,174,52,168,166,33,35,246,213,232,110,216,15,60,189,226,162,152,1,105,36,83,39,37,202,228,56,199,75,154,114,209,134,29,72,169,229,217,128,7,220,4,153,159,3,40,132,131,5,170,119,198,26,76,250,112,211,221,149,2,141,200,71,11,14,218,97,150,140,219,137,23,86,107,205,138,53,45,101,241,113,252,155,80,63,244,253,124,115,248,180,129,116,90,135,16,28,59,58,142,94,167,102,91,156,120,108,89,234,0,21,191,82,106,122,24],
	[41,2,139,71,95,144,235,158,129,211,10,151,123,98,248,185,61,114,255,200,73,63,181,227,231,104,17,251,192,189,137,228,223,101,244,237,50,241,216,136,100,44,177,212,18,246,143,202,33,40,109,225,152,166,11,74,107,105,254,84,106,111,197,89,193,173,3,79,142,91,194,65,138,146,82,247,9,99,239,103,68,174,132,169,8,220,54,131,157,96,22,35,183,28,45,60,25,249,59,153,56,217,199,85,70,52,150,178,120,29,164,67,81,160,179,90,15,69,240,154,62,115,213,43,86,7,83,12,229,168,133,77,108,206,26,198,116,130,167,215,175,221,161,148,214,204,233,36,134,210,187,182,6,159,162,209,135,163,208,5,125,126,21,76,4,102,172,207,39,224,234,110,128,180,245,47,55,186,226,252,42,97,57,14,145,93,87,64,155,32,184,119,165,113,30,124,156,222,75,242,66,37,112,147,72,236,188,149,196,230,122,53,80,121,243,232,201,118,88,171,140,19,127,16,31,51,46,191,27,13,24,203,219,92,170,195,253,250,48,78,190,94,176,117,0,1,38,20,141,23,218,205,49,58,238,34],
];

static ISBOXES: [[u8; SBOX_LENGTH]; NUM_SBOXES] = [
	[240, 134, 199, 100, 98, 247, 87, 80, 158, 250, 222, 150, 76, 35, 140, 99, 231, 50, 234, 0, 71, 116, 182, 225, 13, 243, 244, 59, 236, 196, 53, 165, 126, 4, 3, 194, 26, 36, 114, 56, 219, 251, 130, 44, 124, 20, 238, 216, 86, 106, 66, 33, 122, 104, 190, 72, 68, 46, 16, 28, 85, 246, 164, 30, 11, 166, 141, 185, 214, 58, 128, 9, 23, 171, 131, 133, 51, 201, 54, 38, 94, 43, 227, 121, 168, 125, 233, 7, 29, 224, 172, 95, 118, 149, 192, 93, 254, 127, 77, 235, 187, 139, 135, 110, 163, 12, 217, 191, 148, 179, 15, 221, 252, 75, 49, 89, 145, 142, 109, 138, 105, 147, 156, 170, 113, 129, 88, 111, 65, 159, 96, 245, 19, 181, 101, 169, 162, 55, 249, 255, 5, 25, 132, 42, 78, 18, 189, 237, 210, 112, 204, 120, 146, 151, 209, 45, 52, 195, 67, 41, 90, 70, 117, 137, 157, 27, 83, 160, 79, 136, 184, 84, 248, 102, 173, 17, 92, 212, 6, 81, 180, 208, 220, 1, 64, 152, 205, 10, 62, 115, 176, 154, 178, 63, 97, 202, 188, 2, 22, 175, 229, 167, 155, 34, 253, 213, 143, 31, 230, 39, 206, 91, 241, 60, 177, 32, 107, 57, 103, 223, 61, 197, 40, 74, 193, 144, 228, 47, 161, 232, 8, 14, 198, 183, 215, 242, 123, 200, 207, 174, 186, 226, 21, 218, 48, 211, 82, 37, 203, 119, 69, 153, 24, 73, 239, 108],
	[103, 202, 246, 131, 245, 90, 197, 40, 14, 114, 58, 111, 175, 206, 166, 132, 159, 28, 237, 84, 229, 34, 119, 89, 74, 59, 157, 168, 172, 113, 65, 1, 24, 62, 88, 11, 80, 77, 68, 152, 3, 83, 162, 201, 190, 143, 106, 209, 247, 195, 193, 55, 204, 108, 70, 37, 210, 196, 19, 23, 46, 92, 248, 99, 100, 18, 208, 160, 35, 82, 52, 13, 43, 142, 115, 186, 149, 240, 254, 120, 154, 169, 91, 250, 140, 161, 234, 128, 150, 73, 244, 36, 141, 98, 97, 218, 71, 182, 155, 117, 6, 57, 212, 191, 214, 183, 232, 66, 48, 176, 25, 51, 223, 112, 226, 16, 21, 158, 134, 27, 253, 33, 163, 17, 126, 0, 47, 61, 86, 64, 215, 50, 7, 5, 207, 130, 205, 63, 199, 171, 230, 122, 101, 125, 147, 188, 170, 216, 181, 233, 249, 9, 26, 124, 10, 123, 203, 96, 217, 127, 146, 145, 198, 221, 228, 38, 236, 75, 200, 12, 194, 184, 235, 139, 53, 151, 67, 219, 174, 187, 30, 4, 173, 192, 29, 72, 239, 231, 109, 116, 45, 227, 41, 189, 107, 185, 56, 148, 102, 165, 224, 110, 180, 179, 243, 31, 8, 242, 129, 54, 79, 178, 177, 137, 138, 135, 15, 251, 121, 211, 49, 93, 238, 39, 94, 164, 167, 241, 136, 44, 20, 2, 156, 252, 104, 69, 95, 22, 78, 225, 105, 133, 87, 222, 213, 153, 81, 60, 118, 42, 220, 85, 76, 255, 144, 32],
	[242, 65, 40, 15, 57, 66, 99, 91, 182, 145, 176, 212, 41, 135, 16, 92, 60, 149, 21, 216, 150, 249, 132, 84, 248, 227, 82, 191, 156, 112, 186, 146, 11, 166, 142, 241, 127, 62, 13, 205, 138, 131, 190, 28, 162, 97, 255, 179, 240, 252, 38, 64, 93, 1, 220, 231, 194, 238, 147, 8, 24, 165, 116, 128, 72, 2, 44, 195, 49, 235, 177, 36, 3, 83, 222, 189, 129, 158, 45, 87, 218, 229, 115, 157, 17, 122, 7, 4, 121, 14, 0, 54, 74, 202, 164, 175, 152, 71, 187, 247, 223, 236, 174, 207, 172, 98, 126, 100, 239, 148, 43, 113, 169, 19, 209, 51, 12, 200, 217, 78, 68, 213, 134, 39, 86, 106, 20, 206, 88, 171, 103, 85, 253, 34, 96, 9, 254, 31, 168, 56, 228, 119, 183, 219, 5, 114, 37, 246, 26, 75, 193, 185, 94, 210, 32, 90, 184, 107, 58, 109, 125, 178, 79, 47, 196, 153, 215, 108, 46, 188, 199, 29, 110, 104, 80, 61, 167, 233, 144, 59, 95, 123, 226, 23, 221, 143, 42, 73, 18, 133, 141, 69, 208, 50, 105, 130, 67, 6, 251, 203, 154, 52, 197, 117, 120, 140, 163, 70, 225, 89, 27, 160, 136, 204, 30, 118, 224, 35, 101, 180, 25, 159, 161, 139, 22, 53, 48, 76, 250, 173, 77, 192, 170, 245, 151, 244, 243, 124, 232, 63, 55, 155, 198, 102, 181, 111, 137, 10, 234, 81, 211, 237, 33, 214, 201, 230],
	[249, 158, 199, 184, 181, 188, 124, 179, 15, 50, 13, 203, 9, 39, 204, 152, 235, 16, 68, 4, 65, 250, 138, 211, 255, 132, 192, 92, 236, 173, 101, 87, 107, 145, 45, 146, 160, 163, 81, 162, 185, 140, 105, 86, 59, 217, 129, 120, 115, 10, 134, 108, 142, 216, 57, 19, 166, 11, 238, 237, 153, 33, 1, 224, 23, 44, 63, 75, 91, 97, 27, 202, 174, 24, 5, 168, 193, 7, 48, 46, 223, 112, 252, 161, 137, 12, 212, 117, 103, 247, 233, 243, 43, 30, 240, 106, 37, 206, 18, 47, 73, 218, 242, 79, 40, 159, 253, 213, 246, 126, 150, 135, 195, 220, 170, 228, 232, 49, 55, 190, 245, 83, 254, 20, 227, 38, 121, 35, 178, 231, 99, 187, 186, 125, 172, 234, 119, 210, 215, 110, 208, 200, 239, 8, 25, 64, 14, 80, 93, 198, 207, 21, 157, 182, 169, 222, 244, 29, 84, 183, 61, 69, 156, 100, 66, 111, 144, 241, 143, 175, 189, 104, 128, 62, 141, 122, 28, 82, 31, 32, 230, 76, 67, 123, 72, 53, 56, 78, 85, 154, 102, 251, 36, 139, 77, 74, 95, 131, 191, 167, 201, 60, 164, 109, 114, 214, 70, 113, 26, 171, 90, 196, 52, 148, 41, 22, 151, 177, 205, 209, 180, 197, 133, 0, 136, 89, 155, 6, 165, 176, 116, 34, 149, 118, 248, 58, 2, 130, 42, 98, 3, 219, 51, 94, 225, 17, 147, 96, 229, 127, 194, 71, 221, 226, 54, 88],
	[244, 245, 1, 66, 164, 159, 152, 125, 84, 76, 10, 54, 127, 229, 183, 116, 223, 26, 44, 221, 247, 162, 90, 249, 230, 96, 134, 228, 93, 109, 194, 224, 189, 48, 255, 91, 147, 201, 246, 168, 49, 0, 180, 123, 41, 94, 226, 175, 238, 252, 36, 225, 105, 211, 86, 176, 100, 182, 253, 98, 95, 16, 120, 21, 187, 71, 200, 111, 80, 117, 104, 3, 204, 20, 55, 198, 163, 131, 239, 67, 212, 112, 74, 126, 59, 103, 124, 186, 218, 63, 115, 69, 233, 185, 241, 4, 89, 181, 13, 77, 40, 33, 165, 79, 25, 57, 60, 56, 132, 50, 171, 61, 202, 193, 17, 121, 136, 243, 217, 191, 108, 213, 210, 12, 195, 160, 161, 222, 172, 8, 137, 87, 82, 130, 148, 156, 39, 30, 72, 2, 220, 248, 68, 46, 5, 184, 73, 203, 143, 207, 106, 11, 52, 99, 119, 188, 196, 88, 7, 153, 113, 142, 154, 157, 110, 192, 53, 138, 129, 83, 234, 219, 166, 65, 81, 140, 242, 42, 107, 114, 173, 22, 151, 92, 190, 15, 177, 150, 206, 29, 240, 227, 28, 64, 70, 235, 208, 62, 135, 102, 19, 216, 47, 231, 145, 251, 133, 167, 158, 155, 149, 9, 43, 122, 144, 139, 38, 101, 250, 232, 85, 141, 197, 32, 169, 51, 178, 23, 31, 128, 209, 24, 215, 146, 170, 6, 205, 35, 254, 78, 118, 37, 199, 214, 34, 174, 45, 75, 14, 97, 237, 27, 179, 236, 58, 18]	
];

static PBOX: [usize; PBOX_LEGNTH] = [15, 6, 19, 20, 28, 11, 27, 16, 0, 14, 22, 25, 4, 17, 30, 9, 1, 7, 23, 13, 31, 26, 2, 8, 18, 12, 29, 5, 21, 10, 3, 24];
static IPBOX: [usize; PBOX_LEGNTH] = [8, 16, 22, 30, 12, 27, 1, 17, 23, 15, 29, 5, 25, 19, 9, 0, 7, 13, 24, 2, 3, 28, 10, 18, 31, 11, 21, 6, 4, 26, 14, 20];

pub fn encrypt(plaintext: &mut Vec<u8>, key: u32) {
	let encrypt_block = if shift_type(key) == 0 {
		encrypt_block_left
	} else {
		encrypt_block_right
	};
	let shift = shift_num(key) as usize;
	let mut top = 0;
	for bottom in (0..plaintext.len() - plaintext.len() % 4).step_by(4) {
		let sbox_index = bottom / 4 % NUM_SBOXES;
		top = bottom + 4;
		let mut block = batoi(&plaintext[bottom..top]);
		encrypt_block(&mut block, sbox_index, key, shift);
		let block = itoba(block);
		for i in 0..4{
			unsafe{*plaintext.get_unchecked_mut(bottom + i) = *block.get_unchecked(i);}
		}
	}
	if plaintext.len() - top == 0 {
		return;
	}
	let bottom = top;
	let sbox_index = bottom / 4 % NUM_SBOXES;
	let top = plaintext.len();
	let mut block = batoi(&plaintext[bottom..top]);
	encrypt_block(&mut block, sbox_index, key, shift);
	let block = itoba(block);
	for i in 0..top - bottom {
		unsafe{*plaintext.get_unchecked_mut(bottom + i) = *block.get_unchecked(i);}
	}
}

pub fn decrypt(ciphertext: &mut Vec<u8>, key: u32) {
	let decrypt_block = if shift_type(key) == 1 {
		decrypt_block_left
	} else {
		decrypt_block_right
	};
	let shift = shift_num(key) as usize;
	let mut top = 0;
	for bottom in (0..ciphertext.len() - ciphertext.len() % 4).step_by(4) {
		let sbox_index = bottom / 4 % NUM_SBOXES;
		top = bottom + 4;
		let mut block = batoi(&ciphertext[bottom..top]);
		decrypt_block(&mut block, sbox_index, key, shift);
		let mut block = itoba(block);
		for i in 0..4{
			unsafe{*ciphertext.get_unchecked_mut(bottom + i) = *block.get_unchecked(i);}
		}
	}
	if ciphertext.len() - top == 0 {
		return;
	}
	let bottom = top;
	let sbox_index = bottom / 4 % NUM_SBOXES;
	let top = ciphertext.len();
	let mut block = batoi(&ciphertext[bottom..top]);
	decrypt_block(&mut block, sbox_index, key, shift);
	let block = itoba(block);
	println!("{:?}", block);
	for i in 0..top - bottom {
		unsafe{*ciphertext.get_unchecked_mut(bottom + i) = *block.get_unchecked(i);}
	}
}

fn encrypt_block_left(block: &mut u32, sbox_index: usize, key: u32, shift: usize) {
	substitute(block, unsafe{*SBOXES.get_unchecked(sbox_index)});
	shift_left(block, shift);
	permute(block, PBOX);
	xor_block(block, key);
}

fn encrypt_block_right(block: &mut u32, sbox_index: usize, key: u32, shift: usize) {
	substitute(block, unsafe{*SBOXES.get_unchecked(sbox_index)});
	shift_right(block, shift);
	permute(block, PBOX);
	xor_block(block, key);
}

fn decrypt_block_left(block: &mut u32, sbox_index: usize, key: u32, shift: usize) {
	xor_block(block, key);
	permute(block, IPBOX);
	shift_left(block, shift);
	substitute(block, unsafe{*ISBOXES.get_unchecked(sbox_index)});
}

fn decrypt_block_right(block: &mut u32, sbox_index: usize, key: u32, shift: usize) {
	xor_block(block, key);
	permute(block, IPBOX);
	shift_right(block, shift);
	substitute(block, unsafe{*ISBOXES.get_unchecked(sbox_index)});
}

fn substitute(block: &mut u32, sub_box: [u8; SBOX_LENGTH]) {
	for i in 0..4 {
        let index = 24 - i*8;
        let val = unsafe{*sub_box.get_unchecked(((*block >> index) & 0xff) as usize)} as u32;
        *block = (*block & ((0xff << index) ^ 0xffffffff)) | (val << index);
    }
}

fn permute(block: &mut u32, sub_box: [usize; PBOX_LEGNTH]) {
	let mut permute: u32 = 0;
	for i in 0..PBOX_LEGNTH {
		let p = (*block >> (31 - unsafe{sub_box.get_unchecked(i)})) & 1;
		permute |= p << (31 - i);
	}
	*block = permute;
}

fn xor_block(block: &mut u32, key: u32) {
	*block ^= key;
}

fn shift_right(block: &mut u32, count: usize) {
	*block = *block << (32 - count) | *block >> count;
}

fn shift_left(block: &mut u32, count: usize) {
	*block = *block << count | *block >> (32 - count);
}

fn shift_type(key: u32) -> u8 {
	(key >> 8 & 1) as u8
}

fn shift_num(key: u32) -> u8 {
	(key >> 8 & 31) as u8
}

fn itoba(i: u32) -> [u8; 4] {
	[
	((i >> 24)) as u8,
	((i >> 16) & 0xff) as u8,
	((i >> 8) & 0xff) as u8,
	((i & 0xff)) as u8,
	]
}

fn batoi(block: &[u8]) -> u32 {
	let mut i: u32 = 0;
	for (index, byte) in block.iter().enumerate() {
		i |= ((*byte as u32) << (24 - 8 * index)) as u32;
	}
	i
}

mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn permute_test() {
	    let mut block: u32 = 0b11100;
	    permute(&mut block, PBOX);
	    assert_ne!(block, 0b11100);
	    permute(&mut block, IPBOX);
	    assert_eq!(block, 0b11100);
	}

	#[test]
	fn shift_right_test() {
		let mut v = 0b10010110;
        shift_right(&mut v, 2);
        assert_eq!(v, 0b10000000000000000000000000100101);
        shift_right(&mut v, 32 - 2);
        assert_eq!(v, 0b10010110);
	}

	#[test]
	fn shift_left_test() {
		let mut v = 0b10010110;
        shift_left(&mut v, 26);
        assert_eq!(v, 0b01011000000000000000000000000010);
        shift_left(&mut v, 32 - 26);
        assert_eq!(v, 0b10010110);
	}

	// #[test]
	// fn test_shift_left_() {
	// 	let mut data = vec![1,2,3,4];
	// 	let once = vec![2,3,4,1];
	// 	let twice = vec![3,4,1,2];
	// 	let thrice = vec![4,1,2,3];
	// 	let fource = vec![1,2,3,4];
 //    	shift_left(&mut data[..], 1);
 //    	assert_eq!(data, once);
 //    	shift_left(&mut data[..], 1);
 //    	assert_eq!(data, twice);
 //    	shift_left(&mut data[..], 1);
 //    	assert_eq!(data, thrice);
 //    	shift_left(&mut data[..], 1);
 //    	assert_eq!(data, fource);

	// 	shift_left(&mut data[..], 2);
 //    	assert_eq!(data, twice);
 //    	shift_left(&mut data[..], 2);
 //    	assert_eq!(data, fource);

	// 	shift_left(&mut data[..], 3);
 //    	assert_eq!(data, thrice);
 //    	shift_left(&mut data[..], 1);
 //    	assert_eq!(data, fource);

 //    	shift_left(&mut data[..], 4);
 //    	assert_eq!(data, fource);
		
	// 	shift_left(&mut data[..], 5);
 //    	assert_eq!(data, once);
 //    	// shift_left(&mut data[..], 3);
 //    	// assert_eq!(data, fource);

	// }

	// #[test]
	// fn test_shift_left_empty() {
	// 	let mut data = vec![];
	// 	shift_left(&mut data[..], 1);
	// 	assert_eq!(data, vec![]);
	// }

	// #[test]
	// fn test_shift_left_one() {
	//     let mut data = vec![1];
	// 	shift_left(&mut data[..], 1);
	// 	assert_eq!(data, vec![1]);
	// }

	// #[test]
	// fn test_shift_left_two() {
	//     let mut data = vec![1, 2];
	// 	shift_left(&mut data[..], 1);
	// 	assert_eq!(data, vec![2, 1]);
	// }

	// #[test]
	// fn test_shift_left_three() {
	//     let mut data = vec![1, 2, 3];
	// 	shift_left(&mut data[..], 1);
	// 	assert_eq!(data, vec![2, 3, 1]);
	// }

	// #[test]
	// fn test_shift_right_() {
	// 	let mut data = vec![1, 2, 3, 4];
	// 	let once = vec![4, 1, 2, 3];
	// 	let twice = vec![3, 4 ,1, 2];
	// 	let thrice = vec![2, 3, 4, 1];
	// 	let fource = vec![1, 2, 3, 4];
 //    	shift_right(&mut data[..], 1);
 //    	assert_eq!(data, once);
 //    	shift_right(&mut data[..], 1);
 //    	assert_eq!(data, twice);
 //    	shift_right(&mut data[..], 1);
 //    	assert_eq!(data, thrice);
 //    	shift_right(&mut data[..], 1);
 //    	assert_eq!(data, fource);

	//     shift_right(&mut data[..], 2);
 //    	assert_eq!(data, twice);
 //    	shift_right(&mut data[..], 2);
 //    	assert_eq!(data, fource);

 //    	shift_right(&mut data[..], 3);
 //    	assert_eq!(data, thrice);
 //    	shift_right(&mut data[..], 1);
 //    	assert_eq!(data, fource);

 //    	shift_right(&mut data[..], 4);
 //    	assert_eq!(data, fource);

 //    	shift_right(&mut data[..], 5);
 //    	assert_eq!(data, once); 	
	// }

	// #[test]
	// fn test_shift_right_empty() {
	// 	let mut data = vec![];
	// 	shift_right(&mut data[..], 1);
	// 	assert_eq!(data, vec![]);
	// }

	// #[test]
	// fn test_shift_right_one() {
	//     let mut data = vec![1];
	// 	shift_right(&mut data[..], 1);
	// 	assert_eq!(data, vec![1]);
	// }

	// #[test]
	// fn test_shift_right_two() {
	//     let mut data = vec![1, 2];
	// 	shift_right(&mut data[..], 1);
	// 	assert_eq!(data, vec![2, 1]);
	// }

	// #[test]
	// fn test_shift_right_three() {
	//     let mut data = vec![1, 2, 3];
	// 	shift_right(&mut data[..], 1);
	// 	assert_eq!(data, vec![3, 1, 2]);
	// }

	// #[test]
	// fn test_permute() {
	// 	let original: [u8; 4] = [0b11010101, 0b10100100, 0b00100000, 0b10100001];
	// 	let want: [u8; 4] = [0b00000000, 0b10000000, 0b11011101, 0b10010111];
	// 	let mut got = original.clone();
	// 	permute(&mut got[..], PBOX);
	// 	assert_eq!(want, got);
	// }

	// #[test]
	// fn test_batoi() {
	//     let ba = vec![0b11010101, 0b10100100, 0b00100000, 0b10100001];
	//     let got = batoi(&ba[..]);
	//     assert_eq!(got, 0b11010101101001000010000010100001 as u32);
	// }

	// #[test]
	// fn test_xor() {
	// 	let key = itoba(6208907);
	//     let plaintext: [u8; 4] = [43, 197, 16, 237];
	//     let mut ciphertext = plaintext.clone();
 //    	xor_block(&mut ciphertext[..], key);
 //    	assert_ne!(ciphertext, plaintext);
 //    	xor_block(&mut ciphertext[..], key);
 //    	assert_eq!(ciphertext, plaintext);
	// }

	#[test]
	fn test_substitution() {
	    let mut data = 1234;
	    let original = data;
	    for sbox in 0..NUM_SBOXES {
	    	substitute(&mut data, SBOXES[sbox]);
		    assert_ne!(data, original);
		    substitute(&mut data, ISBOXES[sbox]);
		    assert_eq!(data, original);	
	    }
	}

	#[test]
	fn encrypt_decrypt_file() {
	    const ORIGINAL: &str = "/Users/chris/Documents/personal/rust/projects/bad_crypto/Cargo.toml";
	    let mut plaintext = fs::read(ORIGINAL).unwrap();
	    let original = plaintext.clone();
	    let key = 12345678;
    	encrypt(&mut plaintext, key);
    	assert_ne!(plaintext, original);
    	decrypt(&mut plaintext, key);
    	assert_eq!(plaintext, original);
	}

	#[test]
	fn encrypt_decrypt_divisible_fault() {
	    let mut plaintext: std::vec::Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
	    let original = plaintext.clone();
	    let key = 12345678;
	    encrypt(&mut plaintext, key);
	    assert_ne!(plaintext, original);
	    decrypt(&mut plaintext, key);
	    assert_eq!(plaintext, original);
	}

	#[test]
	fn encrypt_decrypt_indivisible_fault_one_off() {
	    let mut plaintext: std::vec::Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
	    let original = plaintext.clone();
	    let key = 12345678;
	    encrypt(&mut plaintext, key);
	    assert_ne!(plaintext, original);
	    decrypt(&mut plaintext, key);
	    assert_eq!(plaintext, original);
	}

	#[test]
	fn encrypt_decrypt_indivisible_fault_two_off() {
	    let mut plaintext: std::vec::Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
	    let original = plaintext.clone();
	    let key = 12345678;
	    encrypt(&mut plaintext, key);
	    assert_ne!(plaintext, original);
	    decrypt(&mut plaintext, key);
	    assert_eq!(plaintext, original);
	}

	#[test]
	fn encrypt_decrypt_indivisible_fault_three_off() {
	    let mut plaintext: std::vec::Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
	    let original = plaintext.clone();
	    let key = 12345678;
	    encrypt(&mut plaintext, key);
	    assert_ne!(plaintext, original);
	    decrypt(&mut plaintext, key);
	    assert_eq!(plaintext, original);
	}
}