# An example of R package using the savvy framework inside a monorepo

See <https://github.com/yutannihilation/savvy/issues/401>

Update R package files:

```sh
savvy-cli update bindings/rpkg crates/foo-r
mv crates/foo-r/api.h bindings/rpkg/src/rust/api.h # Workaround for now
```

Load the package:

```r
pkgload::load_all("bindings/rpkg")
to_upper("hello")
#> [1] "HELLO"
```
