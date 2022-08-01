# Level 1 - Personal Vault

Let's get ready to write your first own exploit. 
We've simplified the contract used in Level 0 a bit - there's no shared vault anymore, the contract only manages personal vaults.
The functionality is still the same: after initializing your account, you can deposit and withdraw SOL from this account.

Each personal wallet account has an authority. This authority is stored in the account data struct:

```rust
pub struct Wallet {
    pub authority: Pubkey
}
```

Only the authority should be able to withdraw funds from a wallet. Can you break this?