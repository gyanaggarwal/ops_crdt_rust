/*
1. SDP behavior
2. anti entropy correctness
3. vector_clock msg generation
4. ignore duplicate msg
5. causally_stable - normal case
6. causally_stable - node not receiving user update
7. causally_stable - node receiving vector_clock msg
*/

use ops_crdt_rust::test_add_mult;
fn main() {
    // SDP behavior
    // causally_stable - normal case
    test_add_mult::test1();
}



