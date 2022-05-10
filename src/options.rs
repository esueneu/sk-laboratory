
use std::env;
use std::process;

pub struct Options {

	pub cif_input     : String,
	//pub cif_author    : String,
	pub cif_chainid   : String,
}

impl Options {
	pub fn new() -> Options {

		let argv : Vec<String> = env::args().collect();
		let argc : usize = argv.len();

		let mut cif_input     : &String = &String::new();
		//let mut cif_author    : &String = &String::from( "yes" );
		let mut cif_chainid   : &String = &String::from( "A" );

		if argc < 3 { show_usage( &argv[ 0 ] ) };
		let mut i : usize = 1;
		while i < argc {
			match argv[ i ].as_str() {
				"--cif-input"   | "-i"  => { i += 1; cif_input     = &argv[ i ]; }
				//"--cif-author"          => { i += 1; cif_author    = &argv[ i ]; }
				"--cif-chainid" | "-c"  => { i += 1; cif_chainid   = &argv[ i ]; }
				"--help"        | "-h"  => { show_usage( &argv[ 0 ] );           }
				_                       => { show_usage( &argv[ 0 ] );           }
			}
			i += 1;
		}

		/*
		match ( *cif_author ).as_str() {
			"yes" | "no" => (),
			_            => show_usage( &argv[ 0 ] ),
		}
		*/

		Options {
			cif_input   : cif_input.to_string(),
			//cif_author  : cif_author.to_string(),
			cif_chainid : cif_chainid.to_string(),
		}
	}

	pub fn show_parameter( &self ) {

		println!( "\nParameter set :"                               );
		println!( "===============================================" );
		println!( "--cif-input     : {}", self.cif_input            );
		//println!( "--cif-author    : {}", self.cif_author           );
		println!( "--cif-chainid   : {}", self.cif_chainid          );
		println!( "===============================================" );
	}
}

fn show_usage( arg : &String ) {

	println!( "Usage: {} [Options] \n\nOptions :\n\n", *arg );

	println!( "    --cif-input   | -i    Input filename in mmCIF format, REQUIRED. " );
	println!( "    --cif-chainid | -c    Chain ID of the input structural information, default 'A'. " );
	/*
	println!( "    --cif-author       Use the Author Sequence ID ('yes' or 'no', default 'yes').
                           If 'yes',
                               * auth_comp_id
                               * auth_asym_id
                               * auth_atom_id
                           in 'atom_site' Data Category are used.
                           If 'no',
                               * label_comp_id
                               * label_asym_id
                               * label_atom_id
                           are used instead." );
	*/
	println!( "    --help        | -h    Print this help, ignore all other arguments. " );
	println!( "\n" );

	process::exit( 1 );
}