# ctrlc-windows

## 1.0.5

### Patch Changes

- 02d3ec1: Fix: Add node 16 to test and upload workflow (node 15 omitted (see PR #28))

## 1.0.3

### Patch Changes

- 21ce6a2: Include the `not-windows.js` file to short circuit the build on
  non-windows platforms.
- 3add0af: Add node 15 and 16 for the binary release workflow

## 1.0.2

### Patch Changes

- e67ba75: do not even invoke `node-pre-gyp` on non-windows platforms as part of
  the install script. This silences warnings that are printed to the
  console for npm

## 1.0.1

### Patch Changes

- 21ae36b: improved error messages when things go wrong

## 0.2.0

### Minor Changes

- 254b553:

## 0.1.3

### Patch Changes

- d6af1a8: Fix release workflow v2

## 0.1.2

### Patch Changes

- 0b0d6cc: Fix release workflow

## 0.1.1

### Patch Changes

- f66aae5: Release workflow
