# Overview

There seems to exist a bug in SurrealDB's rust client for beta 1.0.0-beta.9 where when you use [update with patch](https://surrealdb.com/docs/integration/libraries/rust#patch) instead of getting the object back as the return value, you get the patch data structure instead.

```
   Compiling surreal_patch_issue v0.1.0 (/home/karim/Projects/surreal_patch_issue)
    Finished dev [unoptimized + debuginfo] target(s) in 5.11s
     Running `target/debug/surreal_patch_issue`
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Api(FromValue { value: Object(Object({"op": Strand(Strand("replace")), "path": Strand(Strand("/done")), "value": True})), error: "missing field `task`" })', src/main.rs:70:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

