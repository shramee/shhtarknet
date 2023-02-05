%lang starknet
from src.main import (
    action_len_store, action_store, jobs_processed_store,
    add_dummy_actions, add_action, shhtarknet_jobs
)
from starkware.cairo.common.cairo_builtins import HashBuiltin

@external
func test_add_dummy_actions{syscall_ptr: felt*, range_check_ptr, pedersen_ptr: HashBuiltin*}() {
    let (actions_len) = action_len_store.read(0);
    add_dummy_actions();
    let (actions_len_after) = action_len_store.read(0);
    assert actions_len_after = actions_len + 20;
    return ();
}

@external
func test_add_action{syscall_ptr: felt*, range_check_ptr, pedersen_ptr: HashBuiltin*}() {
    let (act_index) = action_len_store.read(0);
    add_action( 3, 2, 'move' );
    let (action) = action_store.read( 0, act_index );
    assert action.x = 3;
    assert action.y = 2;
    assert action.action = 'move';
    return ();
}

@external
func test_add_action_count{syscall_ptr: felt*, range_check_ptr, pedersen_ptr: HashBuiltin*}() {
    let (actions_len) = action_len_store.read(0);
    add_action( 1, 2, 'move' );
    add_action( 2, 3, 'jump' );
    let (actions_len_after) = action_len_store.read(0);
    assert actions_len_after = actions_len + 2;
    return ();
}
