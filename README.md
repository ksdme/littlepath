#### Description

This utility allows you to resolve paths using directory structure and fuzzy search. For example, littlepath will be able to resolve the query `~/gh/lph/src` to `~/github/littlepath/src`.

<div align="center">
  <a href="https://asciinema.org/a/499112" target="_blank">
    <img src="https://asciinema.org/a/499112.svg" />
  </a>
</div>

#### Using it with `cd`

Once you have a compiled binary of littlepath in your `$PATH`, you can add something like the following to your shell profile (.bashrc or .zshrc).

```bash
function lcd() {
  cd "$(littlepath --first $1)";
}
```

#### Caveats

Your littlepath query could resolve to multiple paths, the program assigns a score to each match and sorts the matches by their score. Sometimes though, multiple matches could be tied with the same score in which case shortest paths are deliberately given a higher score. The reason behind this is, you can always add more characters to your query to force it to match with a longer path but, the contrary isn't possible.
