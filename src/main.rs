use std::f32;
use std::thread::sleep_ms;
use std::io::{self, Write};

fn main() {
    let mut matrix_old: [[f32; 10]; 10] = [[0.0; 10]; 10];
	let mut matrix_new: [[f32; 10]; 10] = [[0.0; 10]; 10];
	let len = matrix_old.len();
	
	matrix_old[0][1] = 9.0;
	
	loop{
		print_matrix(matrix_old);
		
		for x in 1..len-1 {
			for y in 1..len-1 {	
				matrix_new[x][y] = (matrix_old[x-1][y] + matrix_old[x][y] + matrix_old[x+1][y] + 
									matrix_old[x][y-1] + matrix_old[x][y+1])/5.0;
			}					
		}
		matrix_old = matrix_new;
	}
}

fn print_matrix(matrix: [[f32; 10]; 10]) {
	let len = matrix.len();
	for x in 0..len {
		for y in 0..len {
			print!("|{:.2}", matrix[x][y]);
		}
		println!("");
	}
	io::stdout().flush();
	sleep_ms(1000);
	print!("\n");
}