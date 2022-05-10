
use wasm_bindgen::prelude::*;

mod distance;
mod error;
mod example;
mod mmcif;
//mod options;

#[wasm_bindgen]
pub fn longest_distance_calculator_in_mmcif( input_file : &str, cif_chainid : &str ) -> String {
	//println!( "A program that calculate the longest distance among residues in a protein structure. " );

	/* (2022/05/09 update:) Make an input file be Vec<&str> as input data. */
	let fin_vec : Vec<&str> = input_file.split( '\n' ).collect();

	/* Set options. */
	//let opts = options::Options::new();
	//opts.show_parameter();

	/* Read mmCIF file and get information. */
	let mut data = mmcif::MmCif::new();
	data.read_cif_info( &( fin_vec ) /*&( opts.cif_input )*/ );
	//println!( "{:?}", data.category_list );

	/*
	for i in 0 .. ( data.atom_site_data ).len() {
		println!( "{:?}", ( data.atom_site_data )[ i ] );
	}
	*/

	/* Read "_atom_site" category and set it's order. */
	data.define_category_order();
	//println!( "{:?}", data.category_order );

	/* Get ATOM and HETATM record line. */
	data.set_data_vector();

	/* Remove "label_alt_id". */
	data.remove_alt_id();

	/* Remove HETATM record line. */
	data.remove_hetatm();

	/*
	print!( "group_PDB\t"     );
	print!( "label_atom_id\t" );
	print!( "label_alt_id\t"  );
	print!( "label_comp_id\t" );
	print!( "label_asym_id\t" );
	//print!( "auth_comp_id\t"  );
	//print!( "auth_asym_id\t"  );
	//print!( "auth_atom_id\t"  );
	print!( "cartn_x\t"       );
	print!( "cartn_y\t"       );
	print!( "cartn_z"         );
	print!( "\n" );
	for i in 0 .. ( data.group_pdb ).len() {
		print!( "{}\t", data.group_pdb    [ i ] );
		print!( "{}\t", data.label_atom_id[ i ] );
		print!( "{}\t", data.label_alt_id [ i ] );
		print!( "{}\t", data.label_comp_id[ i ] );
		print!( "{}\t", data.label_asym_id[ i ] );
		//print!( "{}\t", data.auth_comp_id [ i ] );
		//print!( "{}\t", data.auth_asym_id [ i ] );
		//print!( "{}\t", data.auth_atom_id [ i ] );
		print!( "{}\t", data.cartn_x      [ i ] );
		print!( "{}\t", data.cartn_y      [ i ] );
		print!( "{}"  , data.cartn_z      [ i ] );
		print!( "\n" );
	}
	*/

	/* Get coordinate information of CA in each residue. */
	/* (2022/05/08 update) : Make Vec<usize> "target_data_index". */

	let target_data_index : Vec<usize> = data.get_res_coord_info( &( cif_chainid.to_string() ) /*&( opts.cif_chainid )*/ );

	/*
	for i in target_data_index.iter() {
		print!( "{}{}\t{}\t{}\t{}\t{}",
			( data.label_comp_id )[ *i ],
			( data.label_seq_id )[ *i ],
			( data.label_asym_id )[ *i ],
			( data.cartn_x )[ *i ],
			( data.cartn_y )[ *i ],
			( data.cartn_z )[ *i ]
		);
		print!( "\n" );
	}
	*/

	let (
		distance,             // : f64;
		longest_data_index_1, // : usize;
		longest_data_index_2  // : usize;
	) = distance::calc_longest_dist (
		&( data.cartn_x ), // : Vec<f64>;
		&( data.cartn_y ), // : Vec<f64>;
		&( data.cartn_z ), // : Vec<f64>;
		&target_data_index // : Vec<usize>;
	);

	let mut result_string : String = String::new();

	//println!( "The longest distance : {:.3} Å", distance                                                           );
	//print!( "{}{} ", ( data.label_comp_id )[ longest_data_index_1 ], ( data.label_seq_id )[ longest_data_index_1 ] );
	//print!( "(Chain {})    ", ( data.label_asym_id )[ longest_data_index_1 ]                                       );
	//print!( "{}    ", ( data.cartn_x )[ longest_data_index_1 ]                                                     );
	//print!( "{}    ", ( data.cartn_y )[ longest_data_index_1 ]                                                     );
	//print!( "{}\n",   ( data.cartn_z )[ longest_data_index_1 ]                                                     );
	//print!( "{}{} ", ( data.label_comp_id )[ longest_data_index_2 ], ( data.label_seq_id )[ longest_data_index_2 ] );
	//print!( "(Chain {})    ", ( data.label_asym_id )[ longest_data_index_2 ]                                       );
	//print!( "{}    ", ( data.cartn_x )[ longest_data_index_2 ]                                                     );
	//print!( "{}    ", ( data.cartn_y )[ longest_data_index_2 ]                                                     );
	//print!( "{}\n",   ( data.cartn_z )[ longest_data_index_2 ]                                                     );

	result_string += &( format!( "The longest distance : {:.3} Å\n", distance ) );

	result_string += &( format!( "{}{}\t", ( data.label_comp_id )[ longest_data_index_1 ], ( data.label_seq_id )[ longest_data_index_1 ] ) );
	result_string += &( format!( "Chain {}\t", ( data.label_asym_id )[ longest_data_index_1 ]                                            ) );
	result_string += &( format!( "{}\t", ( data.cartn_x )[ longest_data_index_1 ]                                                        ) );
	result_string += &( format!( "{}\t", ( data.cartn_y )[ longest_data_index_1 ]                                                        ) );
	result_string += &( format!( "{}\n",   ( data.cartn_z )[ longest_data_index_1 ]                                                      ) );
	result_string += &( format!( "{}{}\t", ( data.label_comp_id )[ longest_data_index_2 ], ( data.label_seq_id )[ longest_data_index_2 ] ) );
	result_string += &( format!( "Chain {}\t", ( data.label_asym_id )[ longest_data_index_2 ]                                            ) );
	result_string += &( format!( "{}\t", ( data.cartn_x )[ longest_data_index_2 ]                                                        ) );
	result_string += &( format!( "{}\t", ( data.cartn_y )[ longest_data_index_2 ]                                                        ) );
	result_string += &( format!( "{}\n",   ( data.cartn_z )[ longest_data_index_2 ]                                                      ) );

	result_string

}
