
use wasm_bindgen::prelude::*;

//use std::process;
//use colored::*;

#[wasm_bindgen(module="/js/error.js")]
extern "C" {
	pub fn error_bomb_js( msg : &str );
}

pub fn error_bomb( arg : &str )
{

	match arg {
		"len_label_atom_id_group_pdb_not_same"    => error_bomb_js( "Lengh of ( label_atom_id ) != Length of ( group_pdb )."     ),
		"len_label_alt_id_group_pdb_not_same"     => error_bomb_js( "Lengh of ( label_alt_id ) != Length of ( group_pdb )."      ),
		"len_label_comp_id_group_pdb_not_same"    => error_bomb_js( "Lengh of ( label_comp_id ) != Length of ( group_pdb )."     ),
		"len_label_asym_id_group_pdb_not_same"    => error_bomb_js( "Lengh of ( label_asym_id ) != Length of ( group_pdb )."     ),
		"len_label_seq_id_group_pdb_not_same"     => error_bomb_js( "Lengh of ( label_seq_id ) != Length of ( group_pdb )."      ),
		"len_auth_comp_id_group_pdb_not_same"     => error_bomb_js( "Lengh of ( auth_comp_id ) != Length of ( group_pdb )."      ),
		"len_auth_asym_id_group_pdb_not_same"     => error_bomb_js( "Lengh of ( auth_asym_id ) != Length of ( group_pdb )."      ),
		"len_auth_atom_id_group_pdb_not_same"     => error_bomb_js( "Lengh of ( auth_atom_id ) != Length of ( group_pdb )."      ),
		"len_auth_seq_id_group_pdb_not_same"      => error_bomb_js( "Lengh of ( auth_seq_id ) != Length of ( group_pdb )."       ),
		"len_cartn_x_group_pdb_not_same"          => error_bomb_js( "Lengh of ( cartn_x ) != Length of ( group_pdb )."           ),
		"len_cartn_y_group_pdb_not_same"          => error_bomb_js( "Lengh of ( cartn_y ) != Length of ( group_pdb )."           ),
		"len_cartn_z_group_pdb_not_same"          => error_bomb_js( "Lengh of ( cartn_z ) != Length of ( group_pdb )."           ),
		"len_res_num_list_res_name_list_not_same" => error_bomb_js( "Length of ( res_num_list ) != Length of ( res_name_list )." ),
		"len_coord_x_list_res_name_list_not_same" => error_bomb_js( "Length of ( coord_x_list ) != Length of ( res_name_list )." ),
		"len_coord_y_list_res_name_list_not_same" => error_bomb_js( "Length of ( coord_y_list ) != Length of ( res_name_list )." ),
		"len_coord_z_list_res_name_list_not_same" => error_bomb_js( "Length of ( coord_z_list ) != Length of ( res_name_list )." ),
		"len_resseq_list_chainid_list_not_same"   => error_bomb_js( "Length of ( resseq_list ) != Length of ( chainid_list )."   ),
		"len_resname_list_chainid_list_not_same"  => error_bomb_js( "Length of ( resname_list ) != Length of ( chainid_list )."  ),
		"len_asa_list_chainid_list_not_same"      => error_bomb_js( "Length of ( asa_list ) != Length of ( chainid_list )."      ),
		"no_such_seq_in_msa"                      => error_bomb_js( "There was not the same sequence in the input MSA."          ),
		"data_len_not_same"                       => error_bomb_js( "Inadequate structural data. Check the input mmCIF file."    ),
		"chainid_not_contained"                   => error_bomb_js( "No such residue chain ID in the input mmCIF file"           ),
		_                                         => error_bomb_js( "Unexpected error occured."                                  ),
	}

}
