## Contributing to `{{ crate name }}`

Thanks for considering contributing! Here are some of the conventions I try to follow.

### Commit names

This is very optional, but I try to start commit messages with a tag indicating what part of the repo the commit actually touches. For example, [`sim: reorder reset`](https://github.com/ut-utp/core/commit/2a893a981b9dc66d751b1f3bc217e78b39c39ed9).

Actually using the tags isn't required but is encouraged. Feel free to introduce new tags when appropriate.

Here's a one-liner to see what tags are used in a repo:
```bash
alias git-log-stat='git log --oneline | cut -d ' ' -f2- | grep '^[^ ]*:.*$' | cut -d ':' -f 1 | sort | uniq -c | sort -hr'
```

### Formatting

This repo _should_ run `rustfmt` in CI. Regardless, we encourage you to run `cargo fmt` locally before submitting a PR.

### Project Structure

{{ details on the project's structure; else delete }}

### Licensing

Unless you say otherwise, all contributions submitted will be licensed as per the LICENSE file in this repo.
