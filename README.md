# anchorpy-idl
Python bindings for Anchor Rust IDL code. This is the successor
for [`anchorpy-core`](https://github.com/kevinheavey/anchorpy-core).
It lives in a separate repo as a separate package because the Anchor v0.30 IDL
is not backwards compatible, so separating the packages is cleaner in case we ever need
to make changes to older IDL code.
