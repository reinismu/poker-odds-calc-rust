# Poker Odd Calculator (Rust)

This is a rewrite of https://github.com/siavashg87/poker-odds-calc (Shout out to you), but in Rust

Notes: Omaha is not done yet. 

I did write few tests, but this shouldn't be used in production in current state.


## Performance

I'm not the most experienced Rust programmer, but I did expect this code to be a lot faster that original written in typescript. To my surpris 1M iterations can be done in 3 seconds in rust and 5 seconds with original. `time cargo run --release --  -p AdKc -p Ac7c -l 100000` vs `npx poker-odds-calc -p AdKc -p Ac7c -l 1000000`. That was a bit disappointing :/

Maybe there is some low hanging fruit to increase performance.

So you know no bit optimizations are done here. 

