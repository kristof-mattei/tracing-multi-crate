Expected output:

```
2025-01-26T19:20:36.634503Z  INFO crate_a: crate-a
2025-01-26T19:20:36.634546Z  INFO crate_b: crate-b
2025-01-26T19:20:36.634554Z TRACE crate_b: crate-b
2025-01-26T19:20:36.634561Z ERROR crate_main: I do!
```

Notice both Crate A's `TRACE` & our `INFO` are not printed, as per the env filter setup.
