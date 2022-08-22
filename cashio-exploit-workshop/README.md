# Cashio Exploit Workshop

The [Cashio](https://rekt.news/cashio-rekt/) hack was one of the biggest hacks
occurred in Solana ecosystem
which allowed the attacker to print infinite amount of $CASH
with a faked collateral.
This repository provides a local environment that you can try writing your own attack for Cashio.
This exploit workshop is influenced by
[Solana Security Workshop](https://workshop.neodyme.io/index.html)
which we highly recommend to finish before trying this one.

**DISCLAIMER:** This tutorial is provided entirely for an educational purpose.
We DO NOT endorse or support any type of illegal activities.

## Steps

1. The main file you will be working on is `poc/src/main.rs`.
   The file contains the code that prepares the mock environment
   with a real Cashio bank filled with assets.
   Take a look at the  provided code to learn how to use the provided `LocalEnv`
   struct.
2. Write your PoC in `execute_poc()` function.
   Your goal is to write code that exploits a logic bug in the Cashio contract
   that allows you to print infinite amount of $CASH
   and use the printed $CASH to steal collaterals (Saber LP token) stored in the bank.
3. Run `make` to check your answer.
   After the initial build of dependencies `cargo run` also works.

## Hints

* [Cashio attack analysis by Soteria](https://www.soteria.dev/post/cashioapp-attack-whats-the-vulnerability-and-how-soteria-detects-it)
* You can check our model answer under [poc](https://github.com/PwnedNoMore/cashio-exploit-workshop/tree/poc) branch

## Who are we?

We are [Pwned No More](https://pwnednomore.org/), a white hat hacker DAO created by and for the best talents to protect our beloved crypto/Web3 world.
