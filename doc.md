

read each file in
[equity-history](https://github.com/stormasm/equity-history)

and create a file similar to this one...   

here is an example of a influxdb file...   
influx-examples/data/temp{1,2}.txt

here is what the influx filename should look like after it gets created.

```
symbol-name volume=31,close=99 1561930347
```

##### things to do   

[Use this example to read the csv file](https://docs.rs/csv/1.1.3/csv/struct.Reader.html#example)

* convert the timestamp in the csv file to the influxdb time format

[ex02.rs](https://github.com/stormasm/rust-examples/blob/master/fileio/examples/ex02.rs) contains a dir reader which reads all of the files in a directory

##### references

[line protocol details](https://v2.docs.influxdata.com/v2.0/reference/syntax/line-protocol/)

[lifetimes note 1](https://www.google.com/search?q=cannot+move+out+of+%60*entry%60+which+is+behind+a+shared+reference&oq=cannot+move+out+of+%60*entry%60+which+is+behind+a+shared+reference&aqs=chrome..69i57.2527j0j7&sourceid=chrome&ie=UTF-8)

https://stackoverflow.com/questions/32338659/cannot-cannot-move-out-of-value-which-is-behind-a-shared-reference-when-unwrappi

##### Core Post

https://users.rust-lang.org/t/cannot-move-out-of-x-which-is-behind-a-shared-reference/33263

https://users.rust-lang.org/t/pass-mutable-reference-to-vector-as-argument/22130

##### Reddit

https://www.reddit.com/r/rust/comments/5s09m7/struct_members_referencing_other_members/https://www.reddit.com/r/rust/comments/5s09m7/struct_members_referencing_other_members/

##### References

https://facility9.com/2016/04/the-basics-of-rust-structs/

https://doc.rust-lang.org/std/cell/index.html

##### gst

[google search terms 1](https://www.google.com/search?q=rust+idiomatic+way+to+pass+vectors+to+functions&oq=rust+idiomatic+way+to+pass+vectors+to+functions&aqs=chrome..69i57j69i64.14637j0j7&sourceid=chrome&ie=UTF-8)

##### Old Rust Book Explanation

https://doc.rust-lang.org/1.29.0/book/first-edition/mutability.html

https://doc.rust-lang.org/1.29.0/book/first-edition/patterns.html

https://doc.rust-lang.org/1.29.0/book/first-edition/references-and-borrowing.html
