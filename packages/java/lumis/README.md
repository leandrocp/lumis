# Lumis for Java

[![Maven Central](https://img.shields.io/maven-central/v/sh.lumis/lumis)](https://central.sonatype.com/artifact/sh.lumis/lumis)

The Java package for [Lumis](https://lumis.sh). It runs on Java 11 or later with no
native library: Lumis and every grammar are built as one WebAssembly module, which
[Endive](https://endive.run) compiles to JVM bytecode when the jar is built.

## Install

```xml
<dependency>
    <groupId>sh.lumis</groupId>
    <artifactId>lumis</artifactId>
    <version>0.1.0</version>
</dependency>
```

```groovy
implementation 'sh.lumis:lumis:0.1.0'
```

Every language and theme ships in that one jar, so there are no parser packages to add.

## Usage

```java
import sh.lumis.Formatter;
import sh.lumis.Lang;
import sh.lumis.Lumis;
import sh.lumis.Theme;

try (var lumis = Lumis.builder().build()) {
    var highlighter = lumis.highlighter()
            .withLang(Lang.JAVASCRIPT)
            .withTheme(Theme.DRACULA)
            .withFormatter(Formatter.HTML_INLINE)
            .build();

    String html = highlighter.highlight("const x = 1").string();
}
```

Building a `Lumis` loads the module, which takes a few seconds, so create one and
reuse it; highlighters are cheap. Calls on one `Lumis` share a single WebAssembly
memory and are not synchronized, so use it from one thread at a time.

- **Language** - `withLang` takes a `Lang`, or a name or alias such as `"c#"`. The Java API does not detect the language: without `withLang` the code is highlighted as plain text.
- **Theme** - `withTheme` takes a `Theme` or its name, and defaults to `github_dark`.
- **Formatter** - `TERMINAL` (the default), `HTML_INLINE`, `HTML_LINKED` and `BBCODE`. The last two emit class names and scope tags instead of colors, so they reject a theme. Formatter options, multi-theme output and custom formatters are not available yet.
- **Result** - `highlight` takes a `String` or UTF-8 bytes and returns a `LumisResult`. Read it with `string()` or `bytes()`; `failure()` is true when the call into the module failed.

## Command line

A [JBang](https://www.jbang.dev) script highlights files with the package:

```bash
jbang https://github.com/leandrocp/lumis/blob/main/packages/java/lumis/jbang/lumis4j.java --theme dracula --format html-inline App.java
```

## Documentation

Languages, themes and what each formatter emits are documented at
[docs.lumis.sh](https://docs.lumis.sh). To build the package from source, see
[CONTRIBUTING.md](../../../CONTRIBUTING.md#testing).

## Acknowledgements

* [Endive](https://endive.run) for the WebAssembly runtime that turns Lumis into JVM bytecode
