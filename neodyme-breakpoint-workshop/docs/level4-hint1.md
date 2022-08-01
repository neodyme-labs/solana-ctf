# Hint 1

Take a look at `Cargo.toml`. We had to use spl-token version 3.1.0, since the bug is not exploitable with spl-token version 3.1.1 and above.
It might be wise to take a look at the changes between those two versions.


<details>
  <summary>Sub-hint: How to diff these versions?</summary>
  Unfortunately, SPL-token is inside a monorepo. This makes diffing via GitHub's web-ui nearly impossible.
  You can, however, look at all recent commits to the SPL-token program by opening the folder and clicking <a href="https://github.com/solana-labs/solana-program-library/commits/master/token/program/src">History.</a>

  To diff every file in SPL-Token via the CLI, you could clone the solana-program-library repo, and then run `git diff token-v3.1.0 token-v3.1.1 -- token/program/src`.
</details>
