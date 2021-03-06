use std::io;
use std::process;


fn main() {
	
	let mut entrada = String::new();
	io::stdin().read_line(&mut entrada).unwrap();		
	let _first_line:Vec<&str> = entrada.split(" ").collect();
		
	let _num_iter:usize = match _first_line[0].trim().parse(){
		Ok(num)=>{
			if num < 2 || num> 10000{
				println!("impossible");
				process::exit(0x0100);
			}
			num
		},
		Err(_)=>{
			println!("impossible");
			process::exit(0x0100)
		},

	};
	let _h:usize = match _first_line[1].trim().parse(){
		Ok(num)=>{
			if num < 1 || num >= 100000{
				println!("impossible");
				process::exit(0x0100)
			}
			num

		},
		Err(_)=>{
			println!("impossible");
			process::exit(0x0100)
		}


	}; 
	let _a:usize = match _first_line[2].trim().parse(){
		Ok(num)=>{
			if num < 1 || num >= 10000{
				println!("impossible");
				process::exit(0x0100)
			}
			num
		},
		Err(_) =>{
			println!("impossible");
			process::exit(0x0100)
		}
	};
	let _b:usize = match _first_line[3].trim().parse(){
		Ok(num)=>{
			if num<1 || num >= 10000{
				println!("impossible");
				process::exit(0x0100)
			}
			num
		},
		Err(_)=>{
			println!("impossible");
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
			Ok(num) =>{ 
				if num >= 100000{
					println!("impossible");
					process::exit(0x0100)
				}
				num
			},
			Err(_)=>{
				println!("impossible");
				process::exit(0x0100)
			}
			
		});
		_coordinate_y.push(match vec_temp[1].trim().parse(){
			Ok(num)=>{
				if num>_h || num >= 100000{
					println!("impossible");
					process::exit(0x0100)
				}
				num
			},
			Err(_)=>{
				println!("impossible");
					process::exit(0x0100)
			}
			

		});
		i=i+1;
		
	}
	for i in 1.. _coordinate_x.len(){
		if _coordinate_x[i]<=_coordinate_x[i-1]{
			println!("impossible");
			process::exit(0x0100);
		}

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
	let mut _r=0;
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
		_r=(_coordinate_x[i]-_coordinate_x[0])/2;
		/*if _r+_coordinate_y[i]> _h {
			println!("lo mandaron a la mierda en el primer ciclo cuando  r= {} en la corrdenada ({},{})",_r,_coordinate_x[i],_coordinate_y[i]);
			println!("impossible");
			process::exit(0x0100);
		}*/
		matrix[i][0] = matrix_bridge[i][0]+matrix_arch[i][0]; 		
	}
	
	for i in 1.._coordinate_y.len(){
/*
		for j in 0.._coordinate_x.len(){
			for w in 0.._coordinate_x.len(){
				println!("{}\t",matrix[w][i]);
			}

		}
*/
		for k in i.._coordinate_x.len(){
			
			if  i==k{
				matrix_bridge[i][i] = matrix_bridge[i][i-1];
				matrix_arch[i][i]=matrix_arch[i][i-1];
				matrix[i][i]=matrix[i][i-1]; 
			}else{
			
			let temp_bridge= matrix_bridge[i][i-1]+_a*(_h-_coordinate_y[k]);
			let temp_arch= matrix_arch[i][i-1]+_b*(usize::pow(_coordinate_x[k]-_coordinate_x[i],2));
				
				if temp_bridge+temp_arch < matrix[k][i-1]{
				
					/*_r=(_coordinate_x[k]-_coordinate_x[i])/2;
						
					if _r+_coordinate_y[k]>_h{
						//println!("lo mandaron a la mierda cuando r= {} en la corrdenada ({},{})",_r,_coordinate_x[k],_coordinate_x[k]);
						//println!("impossible");
						//process::exit(0x0100);
						
					}
					*/
					matrix_bridge[k][i]=temp_bridge;
					matrix_arch[k][i]=temp_arch;
					/*for j in i..k{

						let x1: isize = (_r+_coordinate_x[i]) as isize;
						let y1: f64 = (_h-_r) as f64;
						let x: isize = _coordinate_x[j] as isize;
						let y: f64 = _coordinate_y[j] as f64;
						let cond:f64  = (isize::pow(_r as isize,2) - isize::pow(x-x1,2))as f64;
						
						if y  >= cond.sqrt() + y1{
						//	println!("Dudo que sea acá pero que no quede");
							println!("impossible");
							process::exit(0x0100);
						}
					}*/
					
					matrix[k][i]=temp_bridge+temp_arch;						
				}else {
					matrix_bridge[k][i]=matrix_bridge[k][i-1];
					matrix_arch[k][i]=matrix_arch[k][i-1];
					matrix[k][i]=matrix[k][i-1];
				}
			}
		}
	}
/*	println!();
	println!("Matriz costo puentes\n");
	
	for i in 0.._coordinate_y.len(){
		for k in 0.._coordinate_x.len(){
			print!("{}\t",matrix_bridge[k][i]);
		}
		println!();
	}

	println!("Matriz costo arcos\n");
	for i in 0.._coordinate_y.len(){
		for k in 0.._coordinate_x.len(){
			print!("{}\t",matrix_arch[k][i]);
		}
		println!();
	}
	
	println!("Matriz suma\n");
	for i in 0.._coordinate_y.len(){
		for k in 0.._coordinate_x.len(){
			print!("{}\t",matrix[k][i]);
		}
		println!();
	}
	
*/	
	
	matrix[_coordinate_x.len()-1][_coordinate_y.len()-1]
}
