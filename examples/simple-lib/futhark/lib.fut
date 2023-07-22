entry average (xs: []f64) = reduce (+) 0 xs / f64.i64 (length xs)

entry double (xs: []f64) = map (* 2) xs

entry swap (a: f64) (b: f64) = (b, a)
