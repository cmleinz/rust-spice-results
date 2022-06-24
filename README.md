# rust-spice-results

This library builds on the work in the [rust-spice library](https://github.com/GregoireHENRY/rust-spice). It adds custom spice errors, and for the functions it wraps, produces rust result types based on the success or failure of the function call.

## Motivation
By default, when calling a function which would produce an error in SPICE, such as:

```
spice::furnsh("");
```

The foreign function call will abort, displaying some code to stdout and panicing the main thread. 

Calling the same function from spice_results we can capture the errors produced by SPICE, and return a `Result<T, SpiceError>`, so the errors can be handled in a more rust-friendly way. 
