# zk Transfer Circuit

The circuit proves knowledge of:
- a hidden amount
- a blinding factor

Such that:
Hash(amount || blinding) == public commitment

No value is revealed on-chain.
Selective disclosure is handled off-circuit.
