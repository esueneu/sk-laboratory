
export function error_bomb_js( arg ) 
{
	let err_msg =
	"\nERROR !!!😱😱😱\n" +
	arg +
	"\nProgram halted !!!💥🔨\n";

	document.getElementById( "outputFile" ).innerHTML
	= err_msg;
}