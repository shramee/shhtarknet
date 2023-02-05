%lang starknet
from starkware.cairo.common.math import assert_nn
from starkware.cairo.common.cairo_builtins import HashBuiltin
from starkware.starknet.common.syscalls import get_caller_address

struct Action {
    x: felt, // Between 0 - 5
    y: felt, // Between 0 - 5
    action: felt, // jump, move
}

@storage_var
func action_len_store( player_id: felt ) -> (count: felt) {
}

@storage_var
func action_store( player_id: felt, action_id: felt ) -> (action: Action) {
}

@storage_var
func jobs_processed_store( player_id: felt, action_id: felt ) -> (action: Action) {
}

@external
func add_action{syscall_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr}(
    x: felt, y: felt, action: felt
) {
    let (caller) = get_caller_address();
    let (index) = action_len_store.read( caller );
    let action_obj = Action( x, y, action );
    action_len_store.write( caller, index + 1 );
    action_store.write(caller, index, action_obj);
    return ();
}

@external
func add_dummy_actions{syscall_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr}() {
    let (caller) = get_caller_address();

    let (index) = action_len_store.read( caller );

    add_action( 6, 5,'move' );
    add_action( 3, 5,'move' );
    add_action( 1, 5,'jump' );
    add_action( 1, 2,'jump' );
    add_action( 1, 4,'move' );
    add_action( 4, 5,'move' );
    add_action( 6, 2,'move' );
    add_action( 2, 5,'move' );
    add_action( 3, 1,'jump' );
    add_action( 1, 5,'jump' );
    add_action( 2, 6,'move' );
    add_action( 3, 2,'move' );
    add_action( 3, 5,'move' );
    add_action( 2, 2,'move' );
    add_action( 6, 5,'move' );
    add_action( 5, 6,'move' );
    add_action( 6, 3,'move' );
    add_action( 5, 5,'move' );
    add_action( 2, 1,'jump' );
    add_action( 4, 3,'move' );

    action_len_store.write( caller, index + 20 );
    return ();
}

@view
func shhtarknet_jobs{syscall_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr}() -> (res: felt) {
    // @TODO Display jobs

    return (1,);
}
