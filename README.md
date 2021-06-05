# GNC [WIP]

[![Build And Test](https://github.com/PAN-Ziyue/GNC/workflows/CI/badge.svg?event=push)](https://github.com/PAN-Ziyue/GNC/actions?workflow=CI)


**GNC** is **N**ot **C**. It is intended for a better and more effective c language.

## Grammar Features

- [x] int main() ...
- [x] declare local int variable
- [x] unary operation  
- [x] binary operation
- [x] conditional statement
- [x] loop statement
- [x] scope 
- [x] function
- [x] global variable
- [x] more types
- [x] cast expression
- [x] pointer
- [x] string
- [ ] array
- [x] `scanf()` and `printf()`
- [ ] struct

## Build

### GNC Cli

```bash
cargo build --package GNC --bin GNC
```

### GNC Online

```bash
yarn
yarn build
```

## Test

### Environment

```bash
# clone THU's minidecaf-test cases as submodule
git submodule init && git submodule update
# install zx
npm i -g zx
```

### Run

```bash
cd test
zx ./minidecaf-test.mjs
```
