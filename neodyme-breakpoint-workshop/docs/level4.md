# Level 4
All the personal vaults we've seen so far only can only store SOL. 
Level 4 now implements a vault for arbitrary SPL tokens, the standard token implementation on Solana.

For each user, the contract manages an SPL token account, to which deposits can be made.
The account is derived from the user's address, and only this user should be able to withdraw the tokens again. 

Can you spot the bug, and steal the tokens from the wallet?

Note: this bug is a bit sneaky, so don't feel bad if you don't spot it right away!