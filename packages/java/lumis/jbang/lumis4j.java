///usr/bin/env jbang "$0" "$@" ; exit $?

// Demo for: https://github.com/leandrocp/lumis/tree/main/packages/java/lumis

//DEPS sh.lumis:lumis:LATEST
//DEPS info.picocli:picocli:4.7.5

import static java.lang.System.*;

import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.Callable;

import sh.lumis.Lumis;
import sh.lumis.Lang;
import sh.lumis.Theme;
import sh.lumis.Formatter;
import picocli.CommandLine;
import picocli.CommandLine.Command;
import picocli.CommandLine.Option;
import picocli.CommandLine.Parameters;

@Command(name = "lumis4j", mixinStandardHelpOptions = true,
    description = "Syntax-highlight files using lumis4j (Tree-sitter + Neovim themes)")
public class lumis4j implements Callable<Integer> {

    @Option(names = "--theme", description = "Theme name (e.g. github_dark, dracula, catppuccin_mocha)")
    String themeName;

    @Option(names = "--format", description = "Output format: terminal, html-inline, html-linked (default: terminal)")
    String formatName;

    @Option(names = "--lang", description = "Language to use (overrides guess from filename); e.g. java, python, rust. Required when reading from stdin (-).")
    String langName;

    @Parameters(arity = "1..*", paramLabel = "FILE", description = "Input files to highlight; use - for stdin")
    List<Path> files;

    private static final Map<String, Lang> EXT_TO_LANG = new HashMap<>();

    static {
        EXT_TO_LANG.put("java", Lang.JAVA);
        EXT_TO_LANG.put("py", Lang.PYTHON);
        EXT_TO_LANG.put("pyw", Lang.PYTHON);
        EXT_TO_LANG.put("js", Lang.JAVASCRIPT);
        EXT_TO_LANG.put("mjs", Lang.JAVASCRIPT);
        EXT_TO_LANG.put("cjs", Lang.JAVASCRIPT);
        EXT_TO_LANG.put("ts", Lang.TYPESCRIPT);
        EXT_TO_LANG.put("tsx", Lang.TSX);
        EXT_TO_LANG.put("go", Lang.GO);
        EXT_TO_LANG.put("rs", Lang.RUST);
        EXT_TO_LANG.put("rb", Lang.RUBY);
        EXT_TO_LANG.put("kt", Lang.KOTLIN);
        EXT_TO_LANG.put("kts", Lang.KOTLIN);
        EXT_TO_LANG.put("sql", Lang.SQL);
        EXT_TO_LANG.put("html", Lang.HTML);
        EXT_TO_LANG.put("htm", Lang.HTML);
        EXT_TO_LANG.put("css", Lang.CSS);
        EXT_TO_LANG.put("scss", Lang.SCSS);
        EXT_TO_LANG.put("json", Lang.JSON);
        EXT_TO_LANG.put("xml", Lang.XML);
        EXT_TO_LANG.put("yaml", Lang.YAML);
        EXT_TO_LANG.put("yml", Lang.YAML);
        EXT_TO_LANG.put("md", Lang.MARKDOWN);
        EXT_TO_LANG.put("markdown", Lang.MARKDOWN);
        EXT_TO_LANG.put("sh", Lang.BASH);
        EXT_TO_LANG.put("bash", Lang.BASH);
        EXT_TO_LANG.put("c", Lang.C);
        EXT_TO_LANG.put("h", Lang.C);
        EXT_TO_LANG.put("cpp", Lang.CPP);
        EXT_TO_LANG.put("cc", Lang.CPP);
        EXT_TO_LANG.put("cxx", Lang.CPP);
        EXT_TO_LANG.put("hpp", Lang.CPP);
        EXT_TO_LANG.put("cs", Lang.CSHARP);
        EXT_TO_LANG.put("csx", Lang.CSHARP);
        EXT_TO_LANG.put("clj", Lang.CLOJURE);
        EXT_TO_LANG.put("cljc", Lang.CLOJURE);
        EXT_TO_LANG.put("ex", Lang.ELIXIR);
        EXT_TO_LANG.put("exs", Lang.ELIXIR);
        EXT_TO_LANG.put("erl", Lang.ERLANG);
        EXT_TO_LANG.put("hrl", Lang.ERLANG);
        EXT_TO_LANG.put("fs", Lang.FSHARP);
        EXT_TO_LANG.put("fsx", Lang.FSHARP);
        EXT_TO_LANG.put("hs", Lang.HASKELL);
        EXT_TO_LANG.put("lhs", Lang.HASKELL);
        EXT_TO_LANG.put("elm", Lang.ELM);
        EXT_TO_LANG.put("lua", Lang.LUA);
        EXT_TO_LANG.put("ml", Lang.OCAML);
        EXT_TO_LANG.put("mli", Lang.OCAML_INTERFACE);
        EXT_TO_LANG.put("ps1", Lang.POWERSHELL);
        EXT_TO_LANG.put("toml", Lang.TOML);
        EXT_TO_LANG.put("vue", Lang.VUE);
        EXT_TO_LANG.put("dart", Lang.DART);
        EXT_TO_LANG.put("swift", Lang.SWIFT);
        EXT_TO_LANG.put("zig", Lang.ZIG);
        EXT_TO_LANG.put("scala", Lang.SCALA);
        EXT_TO_LANG.put("sc", Lang.SCALA);
        EXT_TO_LANG.put("php", Lang.PHP);
        EXT_TO_LANG.put("pl", Lang.PERL);
        EXT_TO_LANG.put("tf", Lang.HCL);
        EXT_TO_LANG.put("graphql", Lang.GRAPHQL);
        EXT_TO_LANG.put("gql", Lang.GRAPHQL);
        EXT_TO_LANG.put("r", Lang.R);
        EXT_TO_LANG.put("asm", Lang.ASM);
        EXT_TO_LANG.put("s", Lang.ASM);
        EXT_TO_LANG.put("nix", Lang.NIX);
        EXT_TO_LANG.put("tex", Lang.LATEX);
        EXT_TO_LANG.put("vim", Lang.VIM);
        EXT_TO_LANG.put("diff", Lang.DIFF);
        EXT_TO_LANG.put("patch", Lang.DIFF);
        EXT_TO_LANG.put("csv", Lang.CSV);
        EXT_TO_LANG.put("proto", Lang.PROTOBUF);
        EXT_TO_LANG.put("eex", Lang.EEX);
        EXT_TO_LANG.put("heex", Lang.HEEX);
        EXT_TO_LANG.put("erb", Lang.ERB);
        EXT_TO_LANG.put("ejs", Lang.EJS);
        EXT_TO_LANG.put("svelte", Lang.SVELTE);
        EXT_TO_LANG.put("astro", Lang.ASTRO);
        EXT_TO_LANG.put("m", Lang.OBJC);
        EXT_TO_LANG.put("mm", Lang.OBJC);
        EXT_TO_LANG.put("gleam", Lang.GLEAM);
        EXT_TO_LANG.put("dockerfile", Lang.DOCKERFILE);
        EXT_TO_LANG.put("makefile", Lang.MAKE);
        EXT_TO_LANG.put("mdx", Lang.MARKDOWN);
    }

    static Lang guessLang(Path path) {
        String name = path.getFileName().toString().toLowerCase();
        if (name.equals("dockerfile") || name.equals("Dockerfile")) return Lang.DOCKERFILE;
        if (name.equals("makefile") || name.equals("Makefile")) return Lang.MAKE;
        int dot = name.lastIndexOf('.');
        if (dot < 0) return Lang.PLAINTEXT;
        String ext = name.substring(dot + 1);
        return EXT_TO_LANG.getOrDefault(ext, Lang.PLAINTEXT);
    }

    static Lang parseLang(String name) {
        String n = name.toLowerCase().replace('-', '_');
        try {
            return Lang.fromString(n);
        } catch (IllegalArgumentException e) {
            return Lang.fromString(n.replace("_", ""));
        }
    }

    static Formatter parseFormat(String name) {
        if (name == null || name.isBlank()) return Formatter.TERMINAL;
        String n = name.trim().toLowerCase().replace("_", "-");
        return switch (n) {
            case "terminal" -> Formatter.TERMINAL;
            case "html-inline" -> Formatter.HTML_INLINE;
            case "html-linked" -> Formatter.HTML_LINKED;
            case "bbcode" -> Formatter.BBCODE;
            default -> throw new IllegalArgumentException("Unknown format: " + name + " (use terminal, html-inline, html-linked, or bbcode)");
        };
    }

    @Override
    public Integer call() throws Exception {
        Theme theme = themeName != null && !themeName.isBlank()
            ? Theme.fromString(themeName.trim().toLowerCase().replace('-', '_'))
            : Theme.GITHUB_DARK;
        Formatter formatter = parseFormat(formatName);
        Lang overrideLang = langName != null && !langName.isBlank()
            ? parseLang(langName.trim())
            : null;

        for (Path file : files) {
            final String source;
            final Lang lang;
            final String label;
            if (file.toString().equals("-")) {
                source = new String(in.readAllBytes(), StandardCharsets.UTF_8);
                lang = overrideLang != null ? overrideLang : Lang.PLAINTEXT;
                label = "stdin";
            } else {
                if (!Files.exists(file)) {
                    err.println("lumis4j: " + file + ": no such file");
                    continue;
                }
                if (Files.isDirectory(file)) {
                    err.println("lumis4j: " + file + ": is a directory");
                    continue;
                }
                lang = overrideLang != null ? overrideLang : guessLang(file);
                source = Files.readString(file);
                label = file.toString();
            }
            try (var lumis = Lumis.builder().build()) {
            var result = lumis.highlighter()
                    .withLang(lang)
                    .withTheme(theme)
                    .withFormatter(formatter)
                    .build().highlight(source);
                out.println("--- " + label + " (" + lang.value() + ") ---");
                out.println(result.string());
            }
        }
        return 0;
    }

    public static void main(String... args) {
        int exitCode = new CommandLine(new lumis4j()).execute(args);
        exit(exitCode);
    }
}
