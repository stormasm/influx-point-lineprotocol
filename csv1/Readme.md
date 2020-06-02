
After reading this
[Readme](https://github.com/stormasm/influx-point-lineprotocol/tree/master/csv1/examples)

Run this command.

```
cre point0
```

To convert the csv files with this date format

```
datetime,open,high,low,close,volume
2015-04-07 00:00,28.16,29.16,27.90,28.40,1589600.00
2015-04-08 00:00,28.91,29.81,28.85,29.18,633500.00
2015-04-09 00:00,29.42,29.99,29.19,29.04,492000.00
```

To this date format

```
datetime,open,high,low,close,volume
2015-04-07 00:00,28.16,29.16,27.90,28.40,1589600.00
2015-04-08 00:00,28.91,29.81,28.85,29.18,633500.00
2015-04-09 00:00,29.42,29.99,29.19,29.04,492000.00
```

Run this command.

```
cre csvc
```
