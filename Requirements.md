#CW1-NEW_LOCKBOX

Put tokens into the contract. If a reset function is not called by the owner in a predefined period,
different supplied accounts can move the tokens out.

##Execute

- CreateLockBox {admin: String, addresses: Vec<String>, expiration: Expiration}
- Reset {id: Uint64}
- Claim {id: Uint64}
- Deposit {id: Uint64}
-

##Query
- Lockbox {id: Uint64}
-
