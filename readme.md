This is an implementation of an [Unlimited Register Machine](https://proofwiki.org/wiki/Definition:Unlimited_Register_Machine) With the following instructions :
- Increment(R1) : Increments the value in register R1.
- Decrement(R1): Decrements the value in register R1
- Zero(R1): Sets the value in register R1 to zero
- Copy(R1,R2): Copies the value in R1 to R2
- Jump[n] : Moves instruction pointer to nth instruction
- Jump(R1)[n] : Sets the instruction pointer to the nth instruction if register R1 contains a zero.

