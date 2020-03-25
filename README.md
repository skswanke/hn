# HN

The HackerNews CLI we deserve, but don't need, or something.

## Usage

Show first 10 results for 'top'
```sh
$ hn
```

Outputs:
```sh
  0: "Private client-side-only PWAs are hard, but now Apple made them impossible"
     125p / 83c - by "soapdog"
  1: "Chrome phasing out support for User-Agent"
     313p / 169c - by "oftenwrong"
  ...
```

Show next 10 results for 'top'
```sh
hn 1
```

Outputs:
```sh
 10: "MeiliSearch: Zero-config alternative to Elasticsearch, made in Rust"
     59p / 15c - by "qdequelen"
 11: "Show HN: To fight quarantine boredom I build a site to watch movies with friends"
     13p / 1c - by "l1am0"
 ...
```

and so on.

Open an article in default browser
```sh
hn -a 1
```

Open comments in default browser
```sh
hn -c 1
```

Open both
```sh
hn -a 1 -c 1
```

Open front page in default browser
```sh
hn -o
```
