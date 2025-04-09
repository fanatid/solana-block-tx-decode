# Optimized block height / tx decoding

```
$ cargo run --release
    Finished `release` profile [optimized] target(s) in 0.13s
     Running `target/release/solana-block-tx-decode`
332254855 | block height 310496508 | solana/prost: 7.529423ms
332254855 | block height 310496508 | solana/prost+convert(agave): 8.951259ms
332254855 | block height 310496508 | manual: 87.884µs
332254855 | tx 42 5on6wqTudPNcZnBo3mH7ybVfe3m4Y6e8c1WHWxeT6HT66Zq76oyAYTXQhGh4e2WXnf1Ar9UVwYpqmJzK9GJD4HdG | solana/prost: 8.052061ms
332254855 | tx 42 5on6wqTudPNcZnBo3mH7ybVfe3m4Y6e8c1WHWxeT6HT66Zq76oyAYTXQhGh4e2WXnf1Ar9UVwYpqmJzK9GJD4HdG | solana/prost+convert(agave): 9.251624ms
332254855 | tx 42 5on6wqTudPNcZnBo3mH7ybVfe3m4Y6e8c1WHWxeT6HT66Zq76oyAYTXQhGh4e2WXnf1Ar9UVwYpqmJzK9GJD4HdG | manual: 109.891µs
332254908 | block height 310496561 | solana/prost: 4.948003ms
332254908 | block height 310496561 | solana/prost+convert(agave): 6.483801ms
332254908 | block height 310496561 | manual: 96.433µs
332254908 | tx 42 2e2NdRFUsVTxVwpRz9mF14u9jMNHm6wkUGhsHuoap12AALVqgJBPjdujBJKDUY3mXaGvXziWnptbUoCH33q28umX | solana/prost: 8.693277ms
332254908 | tx 42 2e2NdRFUsVTxVwpRz9mF14u9jMNHm6wkUGhsHuoap12AALVqgJBPjdujBJKDUY3mXaGvXziWnptbUoCH33q28umX | solana/prost+convert(agave): 9.007209ms
332254908 | tx 42 2e2NdRFUsVTxVwpRz9mF14u9jMNHm6wkUGhsHuoap12AALVqgJBPjdujBJKDUY3mXaGvXziWnptbUoCH33q28umX | manual: 119.077µs
```
