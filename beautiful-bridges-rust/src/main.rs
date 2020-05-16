use std::io;
use std::process;


fn main() {
	
	let mut entrada = String::new();
	io::stdin().read_line(&mut entrada).unwrap();		
	let _first_line:Vec<&str> = entrada.split(" ").collect();
		
	let _num_iter:usize = match _first_line[0].trim().parse(){
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
	let _h:usize = match _first_line[1].trim().parse(){
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
	let _a:usize = match _first_line[2].trim().parse(){
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
	let _b:usize = match _first_line[3].trim().parse(){
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
	let mut _coordinate_x:Vec<usize>=Vec::with_capacity(_num_iter);
	let mut _coordinate_y:Vec<usize>=Vec::with_capacity(_num_iter);	
	let mut i :usize= 0;
	while i<_num_iter {
		let mut temp:String= String::new();
		io::stdin().read_line(&mut temp).unwrap();
		let vec_temp:Vec<&str>=temp.split(" ").collect();
		_coordinate_x.push(match vec_temp[0].trim().parse(){
			Ok(num) => num,
			Err(_)=>{
				println!("Impossible, coordinates should be numbers, and it should be higher than 0");
				process::exit(0x0100)
			}
			
		});
		_coordinate_y.push(match vec_temp[1].trim().parse(){
			Ok(num)=>{
				if num>_h{
					println!("Impossible, y coordinate can't be higher than high");
					process::exit(0x0100)
				}
				num
			},
			Err(_)=>{
				println!("Impossible, coordinates should be numbersm, and it should be higher than 0");
					process::exit(0x0100)
			}
			

		});
		i=i+1;
	}
	let ojala = find_path(_coordinate_x,_coordinate_y,_h,_a,_b);
	println!("{}",ojala);
}

fn find_path(_coordinate_x:Vec<usize>,_coordinate_y:Vec<usize>,_h:usize,_a:usize,_b:usize)->usize{
	let mut matrix:Vec<Vec<usize>> = Vec::new();	
	matrix.resize(_coordinate_y.len(),Vec::new());
	let mut matrix_bridge:Vec<Vec<usize>> = Vec::new();
	matrix_bridge.resize(_coordinate_y.len(),Vec::new());
	let mut matrix_arch:Vec<Vec<usize>> = Vec::new();
	matrix_arch.resize(_coordinate_y.len(),Vec::new());
	
	matrix[0].resize(_coordinate_y.len(),0);
	matrix_bridge[0].resize(_coordinate_y.len(),0);
	matrix_arch[0].resize(_coordinate_y.len(),0);
	
	matrix_bridge[0][0]=_a*(_h-_coordinate_y[0]);
	matrix_arch[0][0]=_b*(usize::pow(_coordinate_x[0],2));
	
	matrix[0][0] = matrix_bridge[0][0] + matrix_arch[0][0];
	for i in 1.._coordinate_x.len(){
		matrix[i].resize(_coordinate_y.len(),0);
		matrix_bridge[i].resize(_coordinate_y.len(),0);
		matrix_arch[i].resize(_coordinate_y.len(),0);
		
		matrix_bridge[i][0]=matrix_bridge[0][0]+_a*(_h-_coordinate_y[i]);
		matrix_arch[i][0]=matrix_arch[0][0]+_b*(usize::pow(_coordinate_x[i]-_coordinate_x[0],2));	
		matrix[i][0] = matrix_bridge[i][0]+matrix_arch[i][0]; 		
	}

	for i in 1.._coordinate_y.len(){

		for k in i.._coordinate_x.len(){
			
			if  i==k{
				matrix_bridge[i][i] = matrix_bridge[i][i-1];
				matrix_arch[i][i]=matrix_arch[i][i-1];
				matrix[i][i]=matrix[i][i-1]; 
			}else{
			
			let temp_bridge= matrix_bridge[i][i]+_a*(_h-_coordinate_y[k]);
			let temp_arch= matrix_arch[i][i]+_b*(usize::pow(_coordinate_x[k]-_coordinate_x[i],2));
				
				if temp_bridge+temp_arch < matrix[k][i-1]{
					matrix_bridge[k][i]=temp_bridge;
					matrix_arch[k][i]=temp_arch;
					matrix[k][i]=temp_bridge+temp_arch;
			
				}else {
					matrix_bridge[k][i]=matrix_bridge[k][i-1];
					matrix_arch[k][i]=matrix_arch[k][i-1];
					matrix[k][i]=matrix[k][i-1];
				}
			}
		}
	}
	
	
	
	for i in 0.._coordinate_y.len(){
		for j in 0.._coordinate_x.len(){
			print! ("{}\t",matrix[j][i]);
		}
		println!();

	}
	matrix[_coordinate_x.len()-1][_coordinate_y.len()-1]
}

