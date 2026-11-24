# Lumis for Java

**Syntax highlighter powered by Tree-sitter and Neovim themes.**

**110+ languages. 200+ themes. Pure Java execution via WebAssembly.**

[![Maven Central](https://img.shields.io/maven-central/v/sh.lumis/lumis)](https://central.sonatype.com/artifact/sh.lumis/lumis)

---

## Features

* **110+ Tree-sitter languages** - Fast and accurate syntax parsing
* **200+ Neovim themes** - Updated and curated themes from the Neovim community
* **Multiple outputs** - HTML (inline/linked), Terminal (ANSI), BBCode
* **Pure Java execution** - Tree-sitter runs in Java via WebAssembly using [Endive](https://endive.run)
* **Zero config** - Works out of the box
* **No native dependencies** - Everything runs in the JVM

## Quick Start

### Maven

```xml
<dependency>
    <groupId>sh.lumis</groupId>
    <artifactId>lumis</artifactId>
    <version>0.1.0</version>
</dependency>
```

### Gradle

```groovy
implementation 'sh.lumis:lumis:0.1.0'
```

### JBang

```java
//DEPS sh.lumis:lumis:0.1.0
```

A command-line script ships with the package:

```bash
jbang https://github.com/leandrocp/lumis/blob/main/packages/java/lumis/jbang/lumis4j.java --theme dracula *.java
```

### Basic Usage

```java
import sh.lumis.Lumis;
import sh.lumis.Lang;
import sh.lumis.Theme;

var lumis = Lumis.builder().build();

var highlighter = lumis.highlighter()
    .withLang(Lang.JAVASCRIPT)
    .withTheme(Theme.DRACULA)
    .build();

var result = highlighter.highlight("console.log('Hello, World!');");
System.out.println(result.string());

lumis.close();
```

### HTML Inline Formatter

```java
import sh.lumis.Lumis;
import sh.lumis.Lang;
import sh.lumis.Theme;
import sh.lumis.Formatter;

var lumis = Lumis.builder().build();

var htmlHighlighter = lumis.highlighter()
    .withLang(Lang.JAVA)
    .withTheme(Theme.CATPPUCCIN_FRAPPE)
    .withFormatter(Formatter.HTML_INLINE)
    .build();

var htmlResult = htmlHighlighter.highlight("public class Hello { }");
System.out.println(htmlResult.string());

lumis.close();
```

### HTML Linked Formatter

```java
import sh.lumis.Lumis;
import sh.lumis.Lang;
import sh.lumis.Formatter;

var lumis = Lumis.builder().build();

var linkedHighlighter = lumis.highlighter()
    .withLang(Lang.RUST)
    .withFormatter(Formatter.HTML_LINKED)
    .build();

var linkedResult = linkedHighlighter.highlight("fn main() {}");
System.out.println(linkedResult.string());

lumis.close();
```

## How It Works

1. **Lumis is compiled to WebAssembly** - The Rust `lumis` crate, with every grammar linked in, is built as one WASI module from the same commit as the Java code
2. **The module is compiled to JVM bytecode** - [Endive](https://endive.run) translates it at build time, so the jar holds plain classes
3. **No native dependencies** - Everything runs inside the JVM, with no JNI or native libraries

This gives you:

- Cross-platform compatibility: it works anywhere Java 11 or later runs
- No native library management
- Memory safety from WebAssembly's sandboxed execution model

## Supported Languages

See the [`Lang`](src/main/java/sh/lumis/Lang.java) enum for the complete list.

## Supported Themes

See the [`Theme`](src/main/java/sh/lumis/Theme.java) enum for the complete list.

## Formatters

| Formatter | Output | Use Case |
|-----------|--------|----------|
| `TERMINAL` | ANSI escape codes | CLI tools, terminal output, logs |
| `HTML_INLINE` | HTML with inline styles | Standalone HTML, email, no external CSS |
| `HTML_LINKED` | HTML with CSS classes | Multiple code blocks, custom styling |
| `BBCODE` | BBCode with scope tags | Forums and other BBCode renderers |

## Building from Source

From the repository root:

```bash
# Build the WebAssembly module from the in-repo lumis crate
mise run setup-java-wasm

# Compile it to bytecode and run the tests
mise run test-java
```

## Acknowledgements

- **[Endive](https://endive.run)** - The pure Java WebAssembly runtime that makes this possible
- **Tree-sitter** - The parsing library behind the highlighting
- **Neovim theme authors** - For the color schemes
