# epoch-calc
### Small utility for dealing with conversions between human readable dates and Unix Timestamps.
 
This program takes a single command line argument, which is expected
to either be a Unix Timestamp or a date/time string. It then either converts the 
timestamp to a human-readable format, or if the input is a date/time string, converts it to a Unix Timestamp. If the input is not recognized, the program prints an error message and exits with a non-zero status code.

## Example Usages
```
$> epoch "8 30 2023 1620"
Unix Timestamp: 1693412400
```

```
$> epoch 1693412400
Human Readable: August 30, 2023 --> 16:20 Hours
```
