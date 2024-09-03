/*
1. SDP behavior
2. anti entropy correctness
3. vector_clock msg generation
4. ignore duplicate msg
5. causally_stable - normal case
6. causally_stable - node not receiving user update
7. causally_stable - node receiving vector_clock msg
8. Refactor EWFlag, DWFlag
9. Refactor AWSet, RWSat
*/

fn main() {
//    ops_crdt_rust::rand_add_mult::test_random();
    ops_crdt_rust::rand_ewflag::test_random();
}



