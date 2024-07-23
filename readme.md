This is an implementation of an [Unlimited Register Machine](https://proofwiki.org/wiki/Definition:Unlimited_Register_Machine)

For implementation reasons, the number of registers isn't unlimited, but only 64.


 It has the following instructions :
- Increment(R1) : Increments the value in register R1.
- Decrement(R1): Decrements the value in register R1
- Zero(R1): Sets the value in register R1 to zero
- Copy(R1,R2): Copies the value in R1 to R2
- Jump[n] : Moves instruction pointer to nth instruction
- JumpIfZero(R1)[n] : Sets the instruction pointer to the nth instruction if register R1 contains a zero.
- Syscall(R1,R2) : Calls the external function (for example 'print') that correspond to the value in R1, with argument the value of R2

These are written as : 

- Increment(R1) : INC R1
- Decrement(R1): DEC R1
- Zero(R1): ZER R1
- Copy(R1,R2): CPY R1 R2
- Jump[n] : JMP n
- Jump(R1)[n] : JMZ R1 n
- Syscall(R1,R2) : SYS R1 R2

The availables syscalls are 

- PrintNumber = 0,
- PrintNumberNL = 1
- PrintChar = 2
- PrintCharNL = 3