# Week 3 -- CKB Rust Learning Report (Lessons 1--8)

## Overview

**Learning track:** Nervos CKB Builder Track -- Week 3\
**Main topics:** Rust fundamentals and CKB Script development\
**Programming language:** Rust\
**Development environment:** Windows\
**Rust version:** 1.100.0\
**Cargo version:** 1.100.0

## Lessons Completed

### Lesson 1 -- Variables and Data Types

Learned Rust variable declaration, mutability, basic data types, and
console output. Practiced working with strings, integers, and Boolean
values.

### Lesson 2 -- Functions, Structs and Enums

Learned how to organize application logic using functions, define custom
data structures with structs, represent possible states with enums, and
handle variants using pattern matching.

### Lesson 3 -- Ownership and Borrowing

Studied Rust's ownership model, move semantics, references, and
borrowing. Learned how Rust manages memory safely and how to avoid
common ownership and borrowing errors.

### Lesson 4 -- Option, Result and Error Handling

Learned how Rust represents optional values and recoverable errors.
Practiced understanding `Option` and `Result`, including their success
and failure variants, and handling outcomes explicitly.

### Lesson 5 -- Collections and Iterators

Learned to work with collections such as vectors and strings, and to
process data using iterators and common iterator operations.

### Lesson 6 -- Modular Rust Code

Studied how to organize Rust applications into modules, separate
implementation across files, control visibility, and import
functionality. This topic forms the basis for maintaining larger Rust
projects.

### Lesson 7 -- First CKB Rust Script

Created a basic Rust-based CKB Script using `ckb-std`. Configured the
RISC-V target and successfully compiled the contract for
`riscv64imac-unknown-none-elf`. Executed the compiled Script using
`ckb-debugger`.

### Lesson 8 -- Script Arguments and Validation

Implemented Script argument loading and validation. Used `load_script()`
to load the current Script and accessed its raw arguments for
validation.

Created automated tests using `ckb-testtool`. The tests deployed the
compiled RISC-V contract into a mock context, constructed transactions
with different Script arguments, and verified the transaction execution
results.

## Testing Results

  -------------------------------------------------------------------------
  Test case           Script arguments  Expected behavior Result
  ------------------- ----------------- ----------------- -----------------
  `test_args_hello`   `hello`           Script accepts    **PASSED**
                                        the arguments;    
                                        verification      
                                        succeeds          

  `test_args_wrong`   `wrong`           Script rejects    **PASSED**
                                        the arguments;    
                                        verification      
                                        fails             
  -------------------------------------------------------------------------

**Automated test summary: 2 passed, 0 failed.**

Test command:

`cargo test --test args -- --nocapture`

## Key Technical Achievements

-   Built a Rust-based CKB Script targeting
    `riscv64imac-unknown-none-elf`.
-   Successfully compiled the contract using Cargo.
-   Executed the compiled Script using `ckb-debugger`.
-   Used `ckb-testtool` to create a mock transaction environment and
    test Script execution.
-   Implemented argument-based validation.
-   Verified both valid and invalid Script argument scenarios through
    automated tests.
-   Separated the RISC-V contract package from the Windows host-side
    test runner to support testing.

## Final Conclusion

During Week 3, I developed a foundation in Rust programming and applied
that knowledge to the development of a Rust-based CKB Script.

The learning journey progressed from Rust fundamentals---including
variables, data types, functions, structures, enums, ownership,
borrowing, error handling, collections, and modular code---to compiling
and executing a CKB Script.

The final exercise demonstrated how a CKB Script can read its arguments
and enforce validation rules. Automated transaction-level tests using
`ckb-testtool` confirmed that valid arguments were accepted and invalid
arguments were rejected, with **2 tests passed and 0 failed**.

This foundation prepares me for the next stage of CKB development:
working with transaction and cell structures, implementing more advanced
on-chain validation rules, and understanding the CKB Script execution
environment in greater depth.
