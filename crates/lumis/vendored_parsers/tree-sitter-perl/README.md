# tree-sitter-perl-better

This is Yet Another perl tree-sitter module.

## Using these bindings

### In General

You can get the built files off of the `release` branch in this repo. If you have specific
instructions for a particular editor, PRs are welcome. We do not store built files on master b/c it 
makes for a very bad DX when you're developing a parser.

We'll happily take PRs needed to get the bindings working in different languages, and we'll update here when they are available via the relevant package managers.


### Neovim

A version of this parser is now part of the nvim-treesitter plugin! Hurrah!

If you'd like to use a version that has not yet made it into nvim-treesitter, you can install these bindings in neovim by using the following snippet.
```lua
local parser_config = require "nvim-treesitter.parsers".get_parser_configs()
parser_config.perl = {
  install_info = {
    url = 'https://github.com/tree-sitter-perl/tree-sitter-perl',
    revision = 'release',
    files = { "src/parser.c", "src/scanner.c" },
  }
}
```

Then you just `:TSInstall perl`. You'll need to copy the queries in the `queries`
directory of this repo into a `queries/perl` directory somewhere in your `rtp`
(`runtimepath`).

See `:h 'rtp'` for more information. Additionally `:echo &rtp` to see your
current `runtimepath`.

### Emacs

As of [Emacs](https://www.gnu.org/software/emacs/) version 29.1, if you have
the tree-sitter library installed, the configure script will automatically
include it in the build.

Once this is done, you can install the Perl bindings
by executing two Emacs lisp forms:

```Emacs Lisp
(setq treesit-language-source-alist
  '((perl . ("https://github.com/tree-sitter-perl/tree-sitter-perl" "release"))))
(treesit-install-language-grammar 'perl)
```

Alternatively, you can run the command interactively:
```
M-x treesit-install-language-grammar <RET>
```
Then answer the prompts accordingly.  Enter `perl` for the language, the
repository URL is `https://github.com/tree-sitter-perl/tree-sitter-perl`
and the branch is `release`.

An Emacs major mode which makes use of these binding ... is yet to be
written.



## Getting Started Developing

To get started, install the dependencies for this repo. The only thing you'll need
installed is node v20 or above (we use more advanced regex features for the unicode
support).

```bash
npm run dev-install
```

That should get you set up with tree-sitter's cli locally. (npm install won't work b/c it
needs the library to be generated to build the bindings so we can use this repo for node
bindings).


### Generating the Bindings

In this project, the generated C source code (stored in the `src` directory) is
.gitignored. In order to generate it, run

```bash
npx tree-sitter generate
```

You'll need to do this after any changes to the grammar.

If you aren't changing the grammar.js file, then you can generate the parser using `src/grammar.json` with whatever version
of the tree-sitter CLI you have installed (no need for node).

```bash
tree-sitter generate src/grammar.json
```

### Running the tests

Tests are stored in the `/test/corpus` directory, as txt files. A little reference on the
syntax can be found [here](https://tree-sitter.github.io/tree-sitter/creating-parsers#command-test).

You can run the tests with

```bash
npx tree-sitter test
```

See the help output (`-h`) for that command for some more details on using the test
runner.

## Contributing

If you'd like to contribute, Pull Requests are welcome! The plan is to build the grammar
from the bottom up, from simple statements with solid code coverage, eventually building
up to full, complex syntax.

You can see a reference of the grammar's DSL [here](https://tree-sitter.github.io/tree-sitter/creating-parsers#the-grammar-dsl). It's fairly straigthforward, and makes for pleasant reading. It is stored in `grammar.js` at the root of this repo.

For subtle points in the grammar implementation, PLEASE leave comments. The extra bytes
spent on the comments in dev will go a long way in the big picture.

### Supporting Scripts

We have a perl script which generates the correct ranges for both the C + JS sides of the
parser. The dependencies are in the `cpanfile` in the root directory. Not necessary unless
you are working on unicode identifiers.

