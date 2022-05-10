
pub fn calc_longest_dist(
	coord_x_list      : &Vec<String>,
	coord_y_list      : &Vec<String>,
	coord_z_list      : &Vec<String>,
	target_data_index : &Vec<usize>
) -> (
	f64,
	usize,
	usize
) {
	let mut longest_dist : f64            = 0.0;
	let mut _longest_data_index_1 : usize = 0;
	let mut _longest_data_index_2 : usize = 0;

	//let vec_len : usize = ( *coord_x_list ).len();
	let vec_len : usize = ( *target_data_index ).len();

	for i in 0 .. ( vec_len - 1 ) {
		let index_i : usize = ( *target_data_index )[ i ];
		for j in ( i + 1 ) .. vec_len {
			let index_j : usize = ( *target_data_index )[ j ];
			let distance : f64 = calc_dist(
				( *coord_x_list )[ index_i ].parse::<f64>().unwrap(),
				( *coord_y_list )[ index_i ].parse::<f64>().unwrap(),
				( *coord_z_list )[ index_i ].parse::<f64>().unwrap(),
				( *coord_x_list )[ index_j ].parse::<f64>().unwrap(),
				( *coord_y_list )[ index_j ].parse::<f64>().unwrap(),
				( *coord_z_list )[ index_j ].parse::<f64>().unwrap()
			);
			if distance > longest_dist {
				longest_dist          = distance;
				_longest_data_index_1 = index_i;
				_longest_data_index_2 = index_j;
			}
		}
	}

	(
		longest_dist,
		_longest_data_index_1,
		_longest_data_index_2
	)

}

pub fn calc_dist(
	x1 : f64,
	y1 : f64,
	z1 : f64,
	x2 : f64,
	y2 : f64,
	z2 : f64 
) -> f64 {

	let distance : f64 = (
		( ( x1 - x2 ) * ( x1 - x2 ) ) +
		( ( y1 - y2 ) * ( y1 - y2 ) ) +
		( ( z1 - z2 ) * ( z1 - z2 ) )
	).sqrt() as f64;

	distance

}