# Bug

The bug is in the `withdraw` function:
```rs
   **wallet_info.lamports.borrow_mut() -= amount;
   **destination_info.lamports.borrow_mut() += amount;
```

can overflow/underflow for large `amount`