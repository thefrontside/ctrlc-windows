---
"ctrlc-windows": patch
---
do not even invoke `node-pre-gyp` on non-windows platforms as part of
the install script. This silences warnings that are printed to the
console for npm
