use std::io;
use std::process;


fn main() {
	
	let mut entrada = String::new();
	io::stdin().read_line(&mut entrada).unwrap();		
	let _first_line:Vec<&str> = entrada.split(" ").collect();
		
	let _num_iter:u64 = match _first_line[0].trim().parse(){
		Ok(num)=>{
			if num < 2 {
				println!("Impossible, first number should be the number of entries, and it should be a number 2≤n");
				process::exit(0x0100);
			}
			num
		},
		Err(_)=>{
			println!("Impossible, first number should be the number of entries, and it should be a number between 2≤n≤10⁴");
			process::exit(0x0100)
		},

	};
	let _h:u64 = match _first_line[1].trim().parse(){
		Ok(num)=>{
			if num < 1 {
				println!("Impossible, second number of the first line, should be the heigh of the bridge an it should be 1≤h");
				process::exit(0x0100)
			}
			num

		},
		Err(_)=>{
			println!("Impossible, second number of the first line should the heigh of the bridge, and it should be a number between 1≤h≤10⁵");
			process::exit(0x0100)
		}


	}; 
	let _a:u64 = match _first_line[2].trim().parse(){
		Ok(num)=>{
			if num <1{
				println!("Impossible, third number of the first line, means the pillars cost");
				process::exit(0x0100)
			}
			num
		},
		Err(_) =>{
			println!("Impossible, third number of the first line, means the pillars cost");
			process::exit(0x0100)
		}
	};
	let _b:u64 = match _first_line[3].trim().parse(){
		Ok(num)=>{
			if num<1{
				println!("Imposiible, fourth number of the first line, means the arch cost");
				process::exit(0x0100)
			}
			num
		},
		Err(_)=>{
			println!("Impossible, fourth number of the first line, means the arch cost");
			process::exit(0x0100);
		}

	};
	let _coordinate_x: Vec<u64>=Vec::new();
	let _coordinate_y: Vec<u64>=Vec::new();	
	let mut temp:String=String::new();
	let vec_temp:Vec<&str>;
	for i in 0.._num_iter-1 {
		io::stdin().read_line(&mut temp).unwrap();
		vec_temp=temp.split(" ").collect();
		_coordinate_x[i] = match vec_temp[0].trim().parse(){
			Ok(num) => num,
			Err(_)=>{
				println!("Impossible, coordinates should be numbers, and it should be higher than 0");
				process::exit(0x0100)
			}
			
		}  
	}

}

