
Leave human2.rs broken so you can see the error message
that was fixed with human3.rs

```
diff human2.rs human3.rs
```

This shows you a major problem you were having with structs
and the simple solution I came up with to solve it.

human4.rs takes care of calling both getlp() inside the
struct and inside the file human4.rs.
