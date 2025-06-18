#!/usr/bin/env just --justfile

default:
    @just --list

list-languages:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo run --bin autumn list-languages

list-themes:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo run --bin autumn list-themes

extract-scopes:
    #!/usr/bin/env bash
    set -euo pipefail
    (cd queries && bash extract_scopes.sh)

update-parsers parser_name="":
    #!/usr/bin/env bash
    set -euo pipefail

    if [[ -z "{{parser_name}}" ]]; then
        echo "⚠️  This will update all parser files in vendored_parsers/"
    else
        echo "⚠️  This will update {{parser_name}} in vendored_parsers/"
    fi
    echo ""
    read -p "Are you sure you want to proceed? (y/N) " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Operation cancelled."
        exit 0
    fi

    TEMP_DIR=$(mktemp -d)
    trap 'rm -rf "$TEMP_DIR"' EXIT

    curl -s https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/main/lua/nvim-treesitter/parsers.lua > "$TEMP_DIR/parsers.lua"

    parsers=(
        "tree-sitter-angular https://github.com/dlvandenberg/tree-sitter-angular.git main"
        "tree-sitter-astro https://github.com/virchau13/tree-sitter-astro.git master"
        "tree-sitter-clojure https://github.com/sogaiu/tree-sitter-clojure.git master"
        "tree-sitter-commonlisp https://github.com/tree-sitter-grammars/tree-sitter-commonlisp.git master"
        "tree-sitter-csv https://github.com/tree-sitter-grammars/tree-sitter-csv.git master"
        "tree-sitter-dart https://github.com/UserNobody14/tree-sitter-dart.git master"
        "tree-sitter-dockerfile https://github.com/camdencheek/tree-sitter-dockerfile.git main"
        "tree-sitter-eex https://github.com/connorlay/tree-sitter-eex.git main"
        "tree-sitter-glimmer https://github.com/ember-tooling/tree-sitter-glimmer.git main"
        "tree-sitter-graphql https://github.com/bkegley/tree-sitter-graphql.git master"
        "tree-sitter-http https://github.com/rest-nvim/tree-sitter-http.git main"
        "tree-sitter-iex https://github.com/elixir-lang/tree-sitter-iex.git main"
        "tree-sitter-kotlin https://github.com/fwcd/tree-sitter-kotlin.git main"
        "tree-sitter-latex https://github.com/latex-lsp/tree-sitter-latex.git master"
        "tree-sitter-liquid https://github.com/hankthetank27/tree-sitter-liquid.git main"
        "tree-sitter-llvm https://github.com/benwilliamgraham/tree-sitter-llvm.git main"
        "tree-sitter-make https://github.com/alemuller/tree-sitter-make.git main"
        "tree-sitter-perl https://github.com/tree-sitter-perl/tree-sitter-perl.git master"
        "tree-sitter-powershell https://github.com/airbus-cert/tree-sitter-powershell.git main"
        "tree-sitter-scss https://github.com/serenadeai/tree-sitter-scss.git master"
        "tree-sitter-surface https://github.com/connorlay/tree-sitter-surface.git main"
        "tree-sitter-vim https://github.com/tree-sitter-grammars/tree-sitter-vim.git master"
        "tree-sitter-vue https://github.com/tree-sitter-grammars/tree-sitter-vue.git main"
    )

    for parser_info in "${parsers[@]}"; do
        read -r parser repo branch <<< "$parser_info"

        # Skip if parser_name is specified and doesn't match current parser
        if [[ -n "{{parser_name}}" ]] && [[ "$parser" != "{{parser_name}}" ]]; then
            continue
        fi

        base_name=${parser#tree-sitter-}
        revision=$(lua -e "
            local parsers = dofile('$TEMP_DIR/parsers.lua')
            local lang_info = parsers['$base_name']
            if lang_info then
                print(lang_info.install_info.revision)
            else
                print('null')
            end
        ")

        echo "🔄 Updating $parser from $repo (revision: $revision)"

        if [ "$revision" = "null" ]; then
            echo "⚠️  No revision found for $parser in nvim-treesitter's lockfile.json, using latest from $branch"
            git clone --depth 1 --branch "$branch" "$repo" "$TEMP_DIR/$parser"
        else
            if ! git clone --depth 1 "$repo" "$TEMP_DIR/$parser" && cd "$TEMP_DIR/$parser" && git fetch origin "$revision" && git checkout "$revision" && cd - > /dev/null; then
                echo "⚠️  Failed to clone specific revision, falling back to latest from $branch"
                git clone --depth 1 --branch "$branch" "$repo" "$TEMP_DIR/$parser"
            fi
        fi

        mkdir -p "vendored_parsers/$parser"

        if [ "$parser" = "tree-sitter-csv" ] && [ -d "$TEMP_DIR/$parser/csv" ]; then
            rm -rf "vendored_parsers/$parser/csv"
            cp -r "$TEMP_DIR/$parser/csv" "vendored_parsers/$parser/"
            echo "✓ Updated $parser"
        elif [ "$parser" = "tree-sitter-latex" ] || [ "$parser" = "tree-sitter-perl" ]; then
            rm -rf "vendored_parsers/$parser"/*
            cp -r "$TEMP_DIR/$parser"/* "vendored_parsers/$parser/"
            (cd "vendored_parsers/$parser" && npm install --no-save tree-sitter-cli && npx tree-sitter generate)
            rm -f "vendored_parsers/$parser/Cargo.toml"
            rm -rf "vendored_parsers/$parser/node_modules"
            rm -rf "vendored_parsers/$parser/bindings"
            echo "✓ Updated $parser"
        elif [ -d "$TEMP_DIR/$parser/src" ]; then
            rm -rf "vendored_parsers/$parser/src"
            cp -r "$TEMP_DIR/$parser/src" "vendored_parsers/$parser/"
            echo "✓ Updated $parser"
        else
            echo "⚠️  No src directory found for $parser"
        fi

        rm -rf "$TEMP_DIR/$parser"
    done

update-queries:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "⚠️  This will regenerate files in queries/"
    echo ""
    read -p "Are you sure you want to proceed? (y/N) " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Operation cancelled."
        exit 0
    fi

    TEMP_DIR=$(mktemp -d)
    git clone --depth 1 --branch "main" https://github.com/nvim-treesitter/nvim-treesitter.git "$TEMP_DIR/nvim-treesitter"

    LANGUAGES=$(find queries -maxdepth 1 -type d | grep -v "^queries$" | sed 's|queries/||')

    for LANG in $LANGUAGES; do
        SRC_DIR="$TEMP_DIR/nvim-treesitter/runtime/queries/$LANG"
        DEST_DIR="queries/$LANG"

        if [ -d "$SRC_DIR" ]; then
            echo "Replacing queries for $LANG"
            mkdir -p "$DEST_DIR"
            cp -r "$SRC_DIR"/* "$DEST_DIR/" 2>/dev/null || true
        else
            echo "No queries found for $LANG in nvim-treesitter"
        fi
    done

    rm -rf "$TEMP_DIR"

gen-theme THEME_NAME:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "⚠️  This will regenerate files in themes/"
    echo ""
    read -p "Do you want to proceed? (y/N) " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Operation cancelled."
        exit 0
    fi

    cd themes
    rm -rf nvim
    nvim --clean --headless -V3 -u init.lua -l extract_theme.lua {{THEME_NAME}}

gen-themes:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "⚠️  This will regenerate files in themes/"
    echo ""
    read -p "Do you want to proceed? (y/N) " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Operation cancelled."
        exit 0
    fi

    find themes -type f -name "*.json" -delete
    cd themes

    THEME_NAMES=$(lua -e "local themes = require('themes'); for _, theme in ipairs(themes) do print(theme.name) end")

    while IFS= read -r THEME_NAME; do
        if [ -n "$THEME_NAME" ]; then
            rm -rf nvim
            nvim --clean --headless -V3 -u init.lua -l extract_theme.lua "$THEME_NAME"
        fi
    done <<< "$THEME_NAMES"

gen-css:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "⚠️  This will regenerate files in css/"
    echo ""
    read -p "Are you sure you want to proceed? (y/N) " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Operation cancelled."
        exit 0
    fi

    find css -type f -name "*.css" -delete
    cargo run --release --features=dev --bin dev gen-css

gen-samples:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "⚠️  This will regenerate files in the samples/ directory."
    echo ""
    read -p "Are you sure you want to proceed? (y/N) " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Operation cancelled."
        exit 0
    fi

    find samples -type f -name "*.html" ! -name "index.html" ! -name "html.html" -delete
    cargo run --release --features=dev --bin dev gen-samples

dev-server:
    #!/usr/bin/env bash
    set -euo pipefail
    (cd samples && npx http-server . -p 8000 --ext-fallback)
