#!/usr/bin/env just --justfile

# List available commands
default:
    @just --list

# Run all tests (Rust crates and Elixir)
test:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Running Rust tests..."
    cargo test --all-features
    echo ""
    echo "Running Elixir tests..."
    cd packages/elixir/lumis && LUMIS_BUILD=1 mix test

# Run all linters (Rust and Elixir)
lint:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Running Rust clippy..."
    cargo clippy --all-features -- -D warnings
    echo ""
    echo "Running Rust fmt check..."
    cargo fmt --all -- --check
    echo ""
    echo "Running Elixir format check..."
    (cd packages/elixir/lumis && mix format --check-formatted)
    echo ""
    echo "Running Elixir compile warnings..."
    (cd packages/elixir/lumis && LUMIS_BUILD=1 mix compile --warnings-as-errors)

# List all supported languages
list-languages:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo run -p lumis list-languages

# List all available themes
list-themes:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo run -p lumis list-themes

# List vendored tree-sitter parsers
list-vendored-parsers:
    #!/usr/bin/env bash
    set -euo pipefail

    TEMP_DIR=$(mktemp -d)
    trap 'rm -rf "$TEMP_DIR"' EXIT

    curl -s https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/main/lua/nvim-treesitter/parsers.lua > "$TEMP_DIR/parsers.lua"

    parsers=(
        "angular"
        "astro"
        "caddy"
        "clojure"
        "commonlisp"
        "csv"
        "dart"
        "dockerfile"
        "eex"
        "fish"
        "glimmer"
        "graphql"
        "kotlin"
        "latex"
        "liquid"
        "llvm"
        "make"
        "markdown"
        "markdown_inline"
        "perl"
        "powershell"
        "scss"
        "surface"
        "typst"
        "vim"
        "vue"
    )

    extra_parsers=(
        "http"
        "iex"
    )

    for base_name in "${parsers[@]}"; do
        parser_info=$(lua -e "
            local parsers = dofile('$TEMP_DIR/parsers.lua')
            local lang_info = parsers['$base_name']
            if lang_info and lang_info.install_info then
                print('$base_name')
            end
        ")
        if [ -n "$parser_info" ]; then
            echo "$base_name"
        fi
    done

    for parser in "${extra_parsers[@]}"; do
        echo "$parser"
    done

# Extract highlight scopes from query files
extract-scopes-highlights:
    #!/usr/bin/env bash
    set -euo pipefail
    find crates/lumis/queries -type f -name "*.scm" -exec grep -oh '@[^_ ][^ ]*' {} \; 2>/dev/null | sed 's/^@//; s/[^a-zA-Z0-9_.-]//g' | sort -u

# Extract highlight scopes from theme files
extract-scopes-themes:
    #!/usr/bin/env bash
    set -euo pipefail
    jq -r '.highlights | keys[]' themes/*.json | sort -u

# Update vendored tree-sitter parsers from upstream
update-vendored-parsers parser_name="":
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

    mapfile -t all_parsers < <(just list-vendored-parsers)

    extra_parsers=(
        "tree-sitter-http https://github.com/rest-nvim/tree-sitter-http.git main"
        "tree-sitter-iex https://github.com/elixir-lang/tree-sitter-iex.git main"
    )

    for base_name in "${all_parsers[@]}"; do
        if [[ "$base_name" == "http" ]] || [[ "$base_name" == "iex" ]]; then
            continue
        fi
        parser="tree-sitter-$base_name"

        if [[ -n "{{parser_name}}" ]] && [[ "$parser" != "{{parser_name}}" ]] && [[ "$base_name" != "{{parser_name}}" ]]; then
            continue
        fi

        parser_info=$(lua -e "
            local parsers = dofile('$TEMP_DIR/parsers.lua')
            local lang_info = parsers['$base_name']
            if lang_info and lang_info.install_info then
                local url = lang_info.install_info.url or ''
                local revision = lang_info.install_info.revision or ''
                local location = lang_info.install_info.location or ''
                print(url .. ' ' .. revision .. ' ' .. location)
            else
                print('null null null')
            end
        ")

        read -r repo revision location <<< "$parser_info"

        if [ "$repo" = "null" ]; then
            echo "⚠️  No parser info found for $base_name in parsers.lua, skipping"
            continue
        fi

        echo "🔄 Updating $parser from $repo (revision: $revision)"

        if [ "$revision" = "null" ] || [ -z "$revision" ]; then
            echo "⚠️  No revision found for $parser, using latest from default branch"
            git clone --depth 1 "$repo" "$TEMP_DIR/$parser"
        else
            if ! git clone "$repo" "$TEMP_DIR/$parser" && cd "$TEMP_DIR/$parser" && git fetch --depth 1 origin "$revision" && git checkout "$revision" && cd - > /dev/null; then
                echo "⚠️  Failed to clone specific revision, falling back to latest"
                rm -rf "$TEMP_DIR/$parser"
                git clone --depth 1 "$repo" "$TEMP_DIR/$parser"
            fi
        fi

        mkdir -p "crates/lumis/vendored_parsers/$parser"

        if [ "$parser" = "tree-sitter-latex" ] || [ "$parser" = "tree-sitter-perl" ]; then
            rm -rf "crates/lumis/vendored_parsers/$parser"/*
            cp -r "$TEMP_DIR/$parser"/* "crates/lumis/vendored_parsers/$parser/"
            (cd "crates/lumis/vendored_parsers/$parser" && npm install --no-save tree-sitter-cli && npx tree-sitter generate)
            rm -f "crates/lumis/vendored_parsers/$parser/Cargo.toml"
            rm -rf "crates/lumis/vendored_parsers/$parser/node_modules"
            rm -rf "crates/lumis/vendored_parsers/$parser/bindings"
            echo "✓ Updated $parser"
        elif [ "$location" != "null" ] && [ -n "$location" ]; then
            if [ -d "$TEMP_DIR/$parser/$location/src" ]; then
                rm -rf "crates/lumis/vendored_parsers/$parser/src"
                cp -r "$TEMP_DIR/$parser/$location/src" "crates/lumis/vendored_parsers/$parser/"
                echo "✓ Updated $parser (with location: $location)"
            else
                echo "⚠️  No src directory found for $parser in location $location"
            fi
        elif [ -d "$TEMP_DIR/$parser/src" ]; then
            rm -rf "crates/lumis/vendored_parsers/$parser/src"
            cp -r "$TEMP_DIR/$parser/src" "crates/lumis/vendored_parsers/$parser/"
            echo "✓ Updated $parser"
        else
            echo "⚠️  No src directory found for $parser"
        fi

        rm -rf "$TEMP_DIR/$parser"
    done

    for parser_line in "${extra_parsers[@]}"; do
        read -r parser repo branch <<< "$parser_line"
        base_name="${parser#tree-sitter-}"

        if [[ -n "{{parser_name}}" ]] && [[ "$parser" != "{{parser_name}}" ]] && [[ "$base_name" != "{{parser_name}}" ]]; then
            continue
        fi

        echo "🔄 Updating extra parser $parser from $repo (branch: $branch)"

        git clone --depth 1 --branch "$branch" "$repo" "$TEMP_DIR/$parser"

        mkdir -p "crates/lumis/vendored_parsers/$parser"

        if [ -d "$TEMP_DIR/$parser/src" ]; then
            rm -rf "crates/lumis/vendored_parsers/$parser/src"
            cp -r "$TEMP_DIR/$parser/src" "crates/lumis/vendored_parsers/$parser/"
            echo "✓ Updated $parser"
        else
            echo "⚠️  No src directory found for $parser"
        fi

        rm -rf "$TEMP_DIR/$parser"
    done

# Update tree-sitter query files from nvim-treesitter
update-queries query_name="":
    #!/usr/bin/env bash
    set -euo pipefail

    if [[ -z "{{query_name}}" ]]; then
        echo "⚠️  This will regenerate files in queries/"
    else
        echo "⚠️  This will regenerate queries for {{query_name}}"
    fi
    echo ""
    read -p "Are you sure you want to proceed? (y/N) " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Operation cancelled."
        exit 0
    fi

    TEMP_DIR=$(mktemp -d)
    git clone --depth 1 --branch "main" https://github.com/nvim-treesitter/nvim-treesitter.git "$TEMP_DIR/nvim-treesitter"

    declare -A special_repos
    special_repos["iex"]="https://github.com/elixir-lang/tree-sitter-iex.git main"
    special_repos["nix"]="https://github.com/nix-community/tree-sitter-nix.git master"
    # https://github.com/leandrocp/lumis/pull/200
    special_repos["python"]="https://github.com/tree-sitter/tree-sitter-python.git master"

    if [[ -n "{{query_name}}" ]]; then
        LANGUAGES="{{query_name}}"
    else
        LANGUAGES=$(find crates/lumis/queries -maxdepth 1 -type d | grep -v "^crates/lumis/queries$" | sed 's|crates/lumis/queries/||')
    fi

    for LANG in $LANGUAGES; do
        DEST_DIR="crates/lumis/queries/$LANG"

        if [[ -n "${special_repos[$LANG]:-}" ]]; then
            IFS=' ' read -r repo branch <<< "${special_repos[$LANG]}"
            echo "🔄 Updating $LANG queries from $repo (branch: $branch)"

            git clone --depth 1 --branch "$branch" "$repo" "$TEMP_DIR/$LANG-special"
            SRC_DIR="$TEMP_DIR/$LANG-special/queries"

            if [ -d "$SRC_DIR" ]; then
                mkdir -p "$DEST_DIR"
                cp -r "$SRC_DIR"/* "$DEST_DIR/" 2>/dev/null || true
                echo "✓ Updated $LANG queries"
            else
                echo "⚠️  No queries found for $LANG in special repo"
            fi

            rm -rf "$TEMP_DIR/$LANG-special"
        else
            SRC_DIR="$TEMP_DIR/nvim-treesitter/runtime/queries/$LANG"

            if [ -d "$SRC_DIR" ]; then
                echo "🔄 Updating $LANG queries from nvim-treesitter"
                mkdir -p "$DEST_DIR"
                cp -r "$SRC_DIR"/* "$DEST_DIR/" 2>/dev/null || true
                echo "✓ Updated $LANG queries"
            else
                echo "⚠️  No queries found for $LANG in nvim-treesitter"
            fi
        fi
    done

    rm -rf "$TEMP_DIR"

# Generate theme JSON files from Neovim colorschemes
gen-themes theme_name="":
    #!/usr/bin/env bash
    set -euo pipefail

    if [[ -z "{{theme_name}}" ]]; then
        echo "⚠️  This will regenerate all theme files in themes/"
    else
        echo "⚠️  This will regenerate {{theme_name}} in themes/"
    fi
    echo ""
    read -p "Do you want to proceed? (y/N) " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Operation cancelled."
        exit 0
    fi

    if [[ -z "{{theme_name}}" ]]; then
        find themes -type f -name "*.json" -delete
    fi

    cd themes

    if [[ -n "{{theme_name}}" ]]; then
        nvim --clean --headless -V3 -u init.lua -l extract_theme.lua "{{theme_name}}"
    else
        THEME_NAMES=$(lua -e "local themes = require('themes'); for _, theme in ipairs(themes) do print(theme.name) end")

        while IFS= read -r THEME_NAME; do
            if [ -n "$THEME_NAME" ]; then
                nvim --clean --headless -V3 -u init.lua -l extract_theme.lua "$THEME_NAME"
            fi
        done <<< "$THEME_NAMES"
    fi

    cd ..
    just sync-themes

# Copy theme JSON files to crates/lumis/themes
sync-themes:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p crates/lumis/themes
    rm -f crates/lumis/themes/*.json || true
    cp themes/*.json crates/lumis/themes/

# Generate CSS files for HTML linked formatter
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
    cargo run -p dev --release gen-css
    just sync-css

# Copy CSS files to crates and packages
sync-css:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p crates/lumis/css
    mkdir -p packages/elixir/lumis/priv/static/css
    rm -f crates/lumis/css/*.css || true
    rm -f packages/elixir/lumis/priv/static/css/*.css || true
    cp css/*.css crates/lumis/css/
    cp css/*.css packages/elixir/lumis/priv/static/css/

# Generate HTML sample files for the website
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
    cargo run -p dev --release gen-samples

# Generate documentation for both Rust and Elixir
docs:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo doc --all-features --no-deps
    (cd packages/elixir/lumis && LUMIS_BUILD=1 mix docs)

# Start local dev server for samples
dev-server:
    #!/usr/bin/env bash
    set -euo pipefail
    (cd samples && npx http-server . -p 8000 --ext-fallback)
