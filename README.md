Rust program to test the checksum algorithm of the Yoroi wallet. Output is below. It shows that 64 other words from the 2048 wordlist create the same checksum, if the last word is changed for the sample passphrase. You can run it by first installing Rust, then start it with `cargo run`.

Can be enhanced to accept a passphrase from command line, and try to recover one unknown word, and with an additional constraint that one address on the blockchain for it is known.

```
$ cargo run
   Compiling maybe-uninit v2.0.0
   Compiling serde v1.0.130
   Compiling bitcoin_hashes v0.9.7
   Compiling rand_core v0.4.2
   Compiling smallvec v0.6.14
   Compiling unicode-normalization v0.1.9
   Compiling bip39 v1.0.1
   Compiling ada-seed-recover v0.1.0 (/home/frank/data/projects/ada-seed-recover)
    Finished dev [unoptimized + debuginfo] target(s) in 4.31s
     Running `target/debug/ada-seed-recover`
found: acoustic
found: alien
found: antenna
found: august
found: badge
found: beyond
found: blossom
found: bronze
found: card
found: cereal
found: chimney
found: coach
found: copy
found: damp
found: delay
found: distance
found: drift
found: dutch
found: enemy
found: erase
found: faint
found: file
found: flat
found: frog
found: gate
found: goat
found: harbor
found: hood
found: increase
found: inspire
found: joke
found: law
found: lock
found: manage
found: mesh
found: mixture
found: movie
found: noodle
found: omit
found: ordinary
found: people
found: plunge
found: profit
found: quick
found: recall
found: resemble
found: return
found: rubber
found: security
found: sheriff
found: slam
found: sort
found: spend
found: start
found: success
found: tag
found: three
found: trash
found: true
found: update
found: vehicle
found: wave
found: west
found: work
original passphrase: sock verb fiction spot repair cotton illness elbow olive core dove elevator van direct bronze
complete wordlist has 2048 words
64 words possible as the last word
```
