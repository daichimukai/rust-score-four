var N = null;var searchIndex = {};
searchIndex["score_four"]={"doc":"Score Four Library for Rust This is a simple Score Four board library for Rust.","items":[[3,"BitBoard","score_four","A bit-board for Score Four",N,N],[12,"0","","",0,N],[3,"Board","","A representation of a score four board",N,N],[4,"Color","","Colors of players to play",N,N],[13,"White","","",1,N],[13,"Black","","",1,N],[4,"BoardStatus","","Status of a board, playing or lines are made",N,N],[13,"Ongoing","","",2,N],[13,"WhiteWin","","",2,N],[13,"BlackWin","","",2,N],[13,"Draw","","",2,N],[11,"eq","","",0,[[["self"],["bitboard"]],["bool"]]],[11,"ne","","",0,[[["self"],["bitboard"]],["bool"]]],[11,"partial_cmp","","",0,[[["self"],["bitboard"]],["option",["ordering"]]]],[11,"lt","","",0,[[["self"],["bitboard"]],["bool"]]],[11,"le","","",0,[[["self"],["bitboard"]],["bool"]]],[11,"gt","","",0,[[["self"],["bitboard"]],["bool"]]],[11,"ge","","",0,[[["self"],["bitboard"]],["bool"]]],[11,"clone","","",0,[[["self"]],["bitboard"]]],[11,"fmt","","",0,[[["self"],["formatter"]],["result"]]],[11,"default","","",0,[[],["bitboard"]]],[11,"new","","Construct a new bitboard instance from u64",0,[[["u64"]],["bitboard"]]],[11,"line_at_pos","","Construct a new bitboard representing a line at the given position",0,[[["u8"]],["bitboard"]]],[11,"get_level_at","","Get the level at the given position",0,[[["self"],["u8"]],["u8"]]],[11,"popcnt","","Count the numbers of beads in this bitboard",0,[[["self"]],["u32"]]],[11,"lined","","Check if a line has constructed",0,[[["self"]],["bool"]]],[11,"row_lined","","Check there exists a lined row or not",0,[[["self"]],["bool"]]],[11,"column_lined","","Check there exists a lined column or not",0,[[["self"]],["bool"]]],[11,"level_lined","","Check there exists a lined column or not",0,[[["self"]],["bool"]]],[11,"bitand","","",0,[[["self"],["bitboard"]],["bitboard"]]],[11,"bitor","","",0,[[["self"],["bitboard"]],["bitboard"]]],[11,"bitand_assign","","",0,[[["self"],["bitboard"]]]],[11,"bitor_assign","","",0,[[["self"],["bitboard"]]]],[11,"not","","",0,[[["self"]],["bitboard"]]],[11,"partial_cmp","","",1,[[["self"],["color"]],["option",["ordering"]]]],[11,"eq","","",1,[[["self"],["color"]],["bool"]]],[11,"clone","","",1,[[["self"]],["color"]]],[11,"fmt","","",1,[[["self"],["formatter"]],["result"]]],[11,"to_index","","",1,[[["self"]],["usize"]]],[11,"not","","",1,[[["self"]],["color"]]],[11,"clone","","",3,[[["self"]],["board"]]],[11,"eq","","",3,[[["self"],["board"]],["bool"]]],[11,"ne","","",3,[[["self"],["board"]],["bool"]]],[11,"fmt","","",3,[[["self"],["formatter"]],["result"]]],[11,"eq","","",2,[[["self"],["boardstatus"]],["bool"]]],[11,"clone","","",2,[[["self"]],["boardstatus"]]],[11,"fmt","","",2,[[["self"],["formatter"]],["result"]]],[11,"new","","Construct a new board",3,[[],["board"]]],[11,"combined","","Combined bitboard of two colors ``` use score_four::{Board, EMPTY};",3,[[["self"]],["bitboard"]]],[11,"side_to_move","","Who's turn?",3,[[["self"]],["color"]]],[11,"put","","Put a bead at the given position ``` use score_four::{Board, Color, BitBoard, EMPTY};",3,[[["self"],["u8"]],["result",["u8","string"]]]],[11,"status","","Put a bead at the given position ``` use score_four::{Board, BoardStatus};",3,[[["self"]],["boardstatus"]]],[11,"default","","",3,[[],["self"]]],[17,"EMPTY","","An empty bitboard.",N,N],[17,"NUM_COLORS","","number of colors",N,N],[11,"to_owned","","",0,[[["self"]],["t"]]],[11,"clone_into","","",0,N],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[["self"]],["u"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"try_into","","",0,[[["self"]],["result"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"get_type_id","","",0,[[["self"]],["typeid"]]],[11,"to_owned","","",3,[[["self"]],["t"]]],[11,"clone_into","","",3,N],[11,"from","","",3,[[["t"]],["t"]]],[11,"into","","",3,[[["self"]],["u"]]],[11,"try_from","","",3,[[["u"]],["result"]]],[11,"borrow","","",3,[[["self"]],["t"]]],[11,"try_into","","",3,[[["self"]],["result"]]],[11,"borrow_mut","","",3,[[["self"]],["t"]]],[11,"get_type_id","","",3,[[["self"]],["typeid"]]],[11,"to_owned","","",1,[[["self"]],["t"]]],[11,"clone_into","","",1,N],[11,"from","","",1,[[["t"]],["t"]]],[11,"into","","",1,[[["self"]],["u"]]],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"try_into","","",1,[[["self"]],["result"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"get_type_id","","",1,[[["self"]],["typeid"]]],[11,"to_owned","","",2,[[["self"]],["t"]]],[11,"clone_into","","",2,N],[11,"from","","",2,[[["t"]],["t"]]],[11,"into","","",2,[[["self"]],["u"]]],[11,"try_from","","",2,[[["u"]],["result"]]],[11,"borrow","","",2,[[["self"]],["t"]]],[11,"try_into","","",2,[[["self"]],["result"]]],[11,"borrow_mut","","",2,[[["self"]],["t"]]],[11,"get_type_id","","",2,[[["self"]],["typeid"]]]],"paths":[[3,"BitBoard"],[4,"Color"],[4,"BoardStatus"],[3,"Board"]]};
initSearch(searchIndex);
