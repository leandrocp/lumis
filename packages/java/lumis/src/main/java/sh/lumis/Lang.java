// Generated from languages.toml by `mise run langs-gen-catalog`. Do not edit.
package sh.lumis;

import java.util.HashMap;
import java.util.Map;

public enum Lang {
    PLAINTEXT(""),
    ANGULAR("angular"),
    ARDUINO("arduino"),
    ASM("asm"),
    ASTRO("astro"),
    BASH("bash"),
    BEND("bend"),
    BICEP("bicep"),
    C("c"),
    CADDY("caddy"),
    CARVE("carve"),
    CLOJURE("clojure"),
    CMAKE("cmake"),
    COMMENT("comment"),
    COMMONLISP("commonlisp"),
    CPP("cpp"),
    CSHARP("csharp"),
    CSS("css"),
    CSV("csv"),
    D("d"),
    DART("dart"),
    DIFF("diff"),
    DOCKERFILE("dockerfile"),
    DOT("dot"),
    EDITORCONFIG("editorconfig"),
    EEX("eex"),
    EJS("ejs"),
    ELIXIR("elixir"),
    ELM("elm"),
    ERB("erb"),
    ERLANG("erlang"),
    FISH("fish"),
    FORTRAN("fortran"),
    FSHARP("fsharp"),
    GITATTRIBUTES("gitattributes"),
    GITIGNORE("gitignore"),
    GLEAM("gleam"),
    GLIMMER("glimmer"),
    GLSL("glsl"),
    GO("go"),
    GRAPHQL("graphql"),
    GROOVY("groovy"),
    HASKELL("haskell"),
    HCL("hcl"),
    HEEX("heex"),
    HTML("html"),
    HTMLDJANGO("htmldjango"),
    HTTP("http"),
    HURL("hurl"),
    IEX("iex"),
    INI("ini"),
    JAVA("java"),
    JAVADOC("javadoc"),
    JAVASCRIPT("javascript"),
    JINJA("jinja"),
    JINJA_INLINE("jinja_inline"),
    JQ("jq"),
    JSON("json"),
    JSON5("json5"),
    JULIA("julia"),
    JUST("just"),
    KDL("kdl"),
    KOTLIN("kotlin"),
    LATEX("latex"),
    LEAN("lean"),
    LIQUID("liquid"),
    LLVM("llvm"),
    LUA("lua"),
    LUADOC("luadoc"),
    MAKE("make"),
    MARKDOWN("markdown"),
    MARKDOWN_INLINE("markdown_inline"),
    MATLAB("matlab"),
    MDX("mdx"),
    MERMAID("mermaid"),
    NGINX("nginx"),
    NIM("nim"),
    NIX("nix"),
    NUSHELL("nushell"),
    OBJC("objc"),
    OCAML("ocaml"),
    OCAML_INTERFACE("ocaml_interface"),
    PASCAL("pascal"),
    PERL("perl"),
    PHP("php"),
    POWERSHELL("powershell"),
    PRISMA("prisma"),
    PROTOBUF("protobuf"),
    PUPPET("puppet"),
    PYTHON("python"),
    QMLJS("qmljs"),
    R("r"),
    RACKET("racket"),
    REGEX("regex"),
    RST("rst"),
    RUBY("ruby"),
    RUST("rust"),
    SCALA("scala"),
    SCHEME("scheme"),
    SCSS("scss"),
    SLIM("slim"),
    SOLIDITY("solidity"),
    SQL("sql"),
    SURFACE("surface"),
    SVELTE("svelte"),
    SWIFT("swift"),
    SYSTEMVERILOG("systemverilog"),
    TCL("tcl"),
    TERRAFORM("terraform"),
    TLAPLUS("tlaplus"),
    TOML("toml"),
    TOON("toon"),
    TSX("tsx"),
    TYPESCRIPT("typescript"),
    TYPST("typst"),
    VHDL("vhdl"),
    VIM("vim"),
    VUE("vue"),
    WAT("wat"),
    WGSL("wgsl"),
    XML("xml"),
    YAML("yaml"),
    ZIG("zig"),
    ZSH("zsh");

    private final String value;

    Lang(String value) {
        this.value = value;
    }

    public String value() {
        return value;
    }

    private static final Map<String, Lang> langs;

    static {
        langs = new HashMap<>();
        for (Lang lang : Lang.values()) {
            langs.put(lang.value(), lang);
        }
        langs.put("plaintext", PLAINTEXT);
        langs.put("text", PLAINTEXT);
        langs.put("txt", PLAINTEXT);
        langs.put("plain", PLAINTEXT);
        langs.put("assembly", ASM);
        langs.put("console", BASH);
        langs.put("sh", BASH);
        langs.put("sh-session", BASH);
        langs.put("shell", BASH);
        langs.put("shell-session", BASH);
        langs.put("terminal", BASH);
        langs.put("cl", COMMONLISP);
        langs.put("c++", CPP);
        langs.put("c#", CSHARP);
        langs.put("flutter", DART);
        langs.put("docker", DOCKERFILE);
        langs.put("leex", EEX);
        langs.put("erl-sh", ERLANG);
        langs.put("erlang-repl", ERLANG);
        langs.put("f#", FSHARP);
        langs.put("ember", GLIMMER);
        langs.put("handlebars", GLIMMER);
        langs.put("golang", GO);
        langs.put("gql", GRAPHQL);
        langs.put("django", HTMLDJANGO);
        langs.put("js", JAVASCRIPT);
        langs.put("jsx", JAVASCRIPT);
        langs.put("jinja2", JINJA);
        langs.put("jsonc", JSON);
        langs.put("jsonld", JSON);
        langs.put("tex", LATEX);
        langs.put("lean4", LEAN);
        langs.put("nu", NUSHELL);
        langs.put("objective-c", OBJC);
        langs.put("pwsh", POWERSHELL);
        langs.put("python3", PYTHON);
        langs.put("qml", QMLJS);
        langs.put("restructuredtext", RST);
        langs.put("psql", SQL);
        langs.put("verilog", SYSTEMVERILOG);
        langs.put("tla", TLAPLUS);
        langs.put("tla+", TLAPLUS);
        langs.put("pluscal", TLAPLUS);
        langs.put("ts", TYPESCRIPT);
        langs.put("viml", VIM);
        langs.put("vimscript", VIM);
        langs.put("wasm", WAT);
        langs.put("webassembly", WAT);
    }

    public static Lang fromString(String lang) {
        if (!langs.containsKey(lang)) {
            throw new IllegalArgumentException("Lang '" + lang + "' not found.");
        }
        return langs.get(lang);
    }
}
