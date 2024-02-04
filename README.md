Resit homework
==============

Edit the lib.rs file, and make sure that all tests pass by using:

```
$ cargo test

...

test tests::active ... ok
test tests::deleted ... ok
test tests::double_delete_impossible ... ok
test tests::subs1 ... ok
```

You must send the `lib.rs` file **alone** for grading.

Grading:
 * if the file does not compile, you will get a failing grade (0)
 * all tests must pass
 * you should make sure you understood the assignment. In particular, this is about making illegal states unrepresentable, so *make sure the types you defined are as restrictive as possible*
 * points will be deducted if the file is incorrectly formatted (you can use `cargo fmt`)
 * points will be deducted for compilation warnings

Have fun!