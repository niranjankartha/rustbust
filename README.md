# rustbust
Fuzzes a website quickly, and in a (mostly) fail-safe way.

```sh
cargo install rustbust
```

## Usage
```sh
rustbust [OPTIONS] <URL> --source <source>
```

where `<source>` is the wordlist that the fuzzer uses.

Example:
```sh
rustbust http://localhost:8080/ --source common.txt
```

### Options
#### `-s`, `--source` (required)
Specifies the wordlist to fuzz from

#### `-o`, `--outfile`
Writes the output (list of hits) to a specified file. Writes to `fuzz.txt` by default.

#### `-p`, `--parallel_count`
Sets the number of parallel requests sent to the server. Defaults to `10`.
