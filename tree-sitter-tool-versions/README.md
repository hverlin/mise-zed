# tree-sitter-tool-versions

A [tree-sitter](https://tree-sitter.github.io/) grammar for `.tool-versions` files, as
[asdf](https://asdf-vm.com/) and [mise](https://mise.jdx.dev/) read them: a tool and its versions
on each line, and `#` comments. Nodes: `tool` (with `name` and `version` fields), `comment`.

```sh
npm install
npm run generate
npm test
```
