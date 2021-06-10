# Usage

- Watch multiple files from a configuration file:

```
$ webtail-rs -c webtail.conf
```

See the [config example](https://github.com/Edu4rdSHL/webtail-rs/blob/master/examples/webtail.conf).

- Watch only a file in an specific port:

```
$ webtail-rs -f file.txt -p 9090
```

- Set an different delay in seconds. Delay is the time it takes for the file updates to be displayed in the browser.

```
$ webtail-rs -f file.txt -p 9090 -d 10 # Updates the content every 10 seconds.
```