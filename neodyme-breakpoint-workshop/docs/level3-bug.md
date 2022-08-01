# Bug

The `Vault` struct can be deserialized into a `TipPool` struct and only the owner of the accounts gets checked in the `withdraw` function.

How can you exploit this?