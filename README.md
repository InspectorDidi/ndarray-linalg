WIP: ndarray-linalg [![Build Status](https://travis-ci.org/termoshtt/ndarray-linalg.svg?branch=master)](https://travis-ci.org/termoshtt/ndarray-linalg)
===============
Linear algebra package for [rust-ndarray](https://github.com/bluss/rust-ndarray)

Examples
---------

```rust
let a = arr2(&[[3.0, 1.0, 1.0], [1.0, 3.0, 1.0], [1.0, 1.0, 3.0]]);
let (e, vecs) = a.eigh();  // eigenvalue and eigenvectors (for Hermite matrix)
```

See complete example at [src/main.rs](src/main.rs).

Progress
---------
- LAPACK bindings using [stainless-steel/lapack](https://github.com/stainless-steel/lapack) (for small matrix):
  - [ ] LU factorization
  - [ ] QR factorization
  - [ ] singular-value decomposition (SVD)
  - [ ] inverse matrix
  - [ ] eigenvalue analysis
- Iterating methods (for large matrix):
  - [ ] SOR
  - [ ] BiCGSTAB
  - [ ] GMRES

Nothing has been done!!