# Introduction

Welcome to the Solana Security Workshop!
Here, we look at Solana smart contracts from an attacker's perspective.
By learning how to find and exploit different types of issues, you'll be able to write more secure contracts as you'll know what to watch out for.

In the first part of the course, we introduce general concepts relevant to the security of Solana contracts and explore one vulnerability in detail.
Next, we've prepared several vulnerable smart contracts as challenges.
Each of these illustrates a different Solana smart contract bug.
You're encouraged to work on exploiting these on your own. If you get stuck, just reach out, are happy to help.

Much of the code you see during this workshop is intentionally vulnerable. Even if the bugs are fixed, the code does not follow good design guidelines. Please do all of us a favor and not use it outside of security demonstrations.

## Requirements

To follow along with this course, you need to be familiar with writing solana contracts and the [Rust](https://rust-lang.org/) programming language.

You also need an environment where you can compile the example contracts and run the attacks.
We have prepared prebuilt environments if you need them, for details, please refer to [Setup](setup.md).

## Who We Are

We started as a group of independent researchers, who love digging into complex concepts and projects. At the end of 2020, we have been introduced into the Solana ecosystem by looking at Solana-core code, in which we have found a number of vulnerabilities. We have since founded the security-research firm [Neodyme](https://neodyme.io), which has been helping the Solana Foundation with peer-reviews of smart contracts.

As such, we have found lots of interesting and critical bugs in smart contracts.
To help make the ecosystem a more secure place, we want to share some insights in this workshop. We hope you enjoy breaking our prepared contracts as much as we do.
