# Fish Shell Script Example

# Variables
set name "World"
set -x PATH $HOME/bin $PATH
set -l local_var "I'm local"

# Print with string interpolation
echo "Hello, $name!"
echo "Current directory: "(pwd)

# Functions
function greet
    set -l who $argv[1]
    echo "Greetings, $who!"
end

function mkcd --description "Create and change to directory"
    mkdir -p $argv[1]
    and cd $argv[1]
end

# Conditionals
if test -f ~/.config/fish/local.fish
    source ~/.config/fish/local.fish
else if test -d ~/.local/bin
    set -x PATH ~/.local/bin $PATH
end

# Switch statement
switch $hostname
    case "*.local"
        echo "Running on a local machine"
    case "prod-*"
        echo "Running in production"
    case "*"
        echo "Unknown environment"
end

# Loops
for file in *.txt
    echo "Processing $file"
end

set items apple banana cherry
for item in $items
    echo "Fruit: $item"
end

# While loop
set count 0
while test $count -lt 5
    echo "Count: $count"
    set count (math $count + 1)
end

# Command substitution
set git_branch (git branch --show-current 2>/dev/null)
set file_count (count *.fish)

# Piping and redirections
cat file.txt | grep "pattern" | sort | uniq > output.txt 2>&1

# Status checks
if command -v nvim >/dev/null
    alias vim nvim
end

# Abbreviations (like aliases but expand)
abbr -a g git
abbr -a gc "git commit"
abbr -a gp "git push"

# Event handlers
function --on-variable PWD --description "Auto ls on cd"
    ls
end

# Exit status
function backup
    cp -r $argv[1] $argv[1].bak
    or return 1
    echo "Backup created"
end
