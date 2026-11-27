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

The language, theme and formatter are optional. The
[Java guide](https://docs.lumis.sh/usage/java) lists their values and defaults.

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
