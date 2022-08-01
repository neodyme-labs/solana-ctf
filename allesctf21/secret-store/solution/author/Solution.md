```
$ solana transaction-history --show-transactions --url http://localhost:1024 "Secret1111111111111111111111111111111111111"
2D2GAvgU5jvC8pmJNHxp5rQReDDXXg8ocPeyUBTs6eQHxYEJwXbkmdTMkhrzogqWK9Ub9QGgTjuVZKc2YaVRsbSE
  Recent Blockhash: G4o7LHo6Jxex2b1G3ocNoZWeKjkiSGqJqqhMk9GswJ2j
  Signature 0: 2D2GAvgU5jvC8pmJNHxp5rQReDDXXg8ocPeyUBTs6eQHxYEJwXbkmdTMkhrzogqWK9Ub9QGgTjuVZKc2YaVRsbSE
  Signature 1: zSRNwYLnqrnnr7HoaR5pKvoPXuJ6pegtgmysuL1oyBDZnXDvU39uwubrYEWo9a1cG5yQ76nPNFDMRZHXxVwgH5f
  Account 0: srw- BueKapuDHHfBcDCw7zB4uZwxb6BWmAFPdA3v8diCspTK (fee payer)
  Account 1: sr-- DtwhRLm3xMDGBnXcLzYygxNrd8Hz2QHbQsNtMGJ1iYsQ
  Account 2: -rw- 4WqgwsyU8WautoDMPQZFrnJ26d7UWrY39GHyz3qWtFQN
  Account 3: -rw- 3GpBPRBeG4gKMjGnB39B8jxfKy7zNgTA2TnKuxojHEm8
  Account 4: -r-- SysvarRent111111111111111111111111111111111
  Account 5: -r-- 11111111111111111111111111111111
  Account 6: -r-- TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
  Account 7: -r-x Secret1111111111111111111111111111111111111
  Instruction 0
    Program:   Secret1111111111111111111111111111111111111 (7)
    Account 0: 4WqgwsyU8WautoDMPQZFrnJ26d7UWrY39GHyz3qWtFQN (2)
    Account 1: BueKapuDHHfBcDCw7zB4uZwxb6BWmAFPdA3v8diCspTK (0)
    Account 2: 3GpBPRBeG4gKMjGnB39B8jxfKy7zNgTA2TnKuxojHEm8 (3)
    Account 3: DtwhRLm3xMDGBnXcLzYygxNrd8Hz2QHbQsNtMGJ1iYsQ (1)
    Account 4: SysvarRent111111111111111111111111111111111 (4)
    Account 5: 11111111111111111111111111111111 (5)
    Account 6: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA (6)
    Data: [0, 114, 33, 13, 253, 126, 138, 181, 140]
  Status: Ok
    Fee: ◎0
    Account 0 balance: ◎500000000 -> ◎499999999.9990534
    Account 1 balance: ◎0
    Account 2 balance: ◎0 -> ◎0.00094656
    Account 3 balance: ◎0.00203928
    Account 4 balance: ◎0.0010092
    Account 5 balance: ◎0.000000001
    Account 6 balance: ◎0.000000001
    Account 7 balance: ◎0.000000001
  Log Messages:
    Program Secret1111111111111111111111111111111111111 invoke [1]
    Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]
    Program log: Instruction: SetAuthority
    Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 2153 of 193996 compute units
    Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success
    Program 11111111111111111111111111111111 invoke [2]
    Program 11111111111111111111111111111111 success
    Program Secret1111111111111111111111111111111111111 consumed 14456 of 200000 compute units
    Program Secret1111111111111111111111111111111111111 success

1 transactions found


$ solana account --url http://localhost:1024 4WqgwsyU8WautoDMPQZFrnJ26d7UWrY39GHyz3qWtFQN

Public Key: 4WqgwsyU8WautoDMPQZFrnJ26d7UWrY39GHyz3qWtFQN
Balance: 0.00094656 SOL
Owner: Secret1111111111111111111111111111111111111
Executable: false
Rent Epoch: 0
Length: 8 (0x8) bytes
0000:   72 21 0d fd  7e 8a b5 8c                             r!..~...

```

```python
import struct
struct.unpack(">Q",struct.pack("<Q", 0x72210dfd7e8ab58c))[0]

>>> 10139162414110548338

$ ./target/debug/store-cli -k ./keys/rich-boi.json get-flag "3GpBPRBeG4gKMjGnB39B8jxfKy7zNgTA2TnKuxojHEm8" 10139162414110548338

[cli/src/main.rs:170] tt.transaction.meta.unwrap().log_messages = Some(
    [
        "ALLES!{🅱lockchain}",
    ],
)
```
