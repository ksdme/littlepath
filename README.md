#### Description

This utility allows you to resolve paths using directory structure and fuzzy search. To address a directory named `~/github/littlepath/src`, you can something like `~/gh/lph/src`.

#### Using it with cd

Once you have a compile binary of littlepatch in your $PATH, you can add something like the following to your shell profile (.bashrc or .zshrc) to use littlepath for changing directories.

```bash
function lcd() {
  cd "$(littlepath --first $1)"
}
```

#### Caveats

Your littlepath could resolve to multiple paths, the program assigns a score to each match and sorts the matches by their score. Sometimes though, multiple matches could be tied for the top position in which case shortest paths are deliberately given a higher score. The reason behind this is, you can always add more characters to your littlepath to force it to match with a longer path and the contrary isn't possible.
