package sh.lumis;

import java.util.HashMap;
import java.util.Map;

public enum Lang {
    PLAINTEXT(""),
    ANGULAR("angular"),
    ARDUINO("arduino"),
    ASSEMBLY("asm"),
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
    COMMON_LISP("commonlisp"),
    CPP("cpp"),
    C_SHARP("csharp"),
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
    F_SHARP("fsharp"),
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
    HTML_DJANGO("htmldjango"),
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
    OBJ_C("objc"),
    OCAML("ocaml"),
    OCAML_INTERFACE("ocaml_interface"),
    PASCAL("pascal"),
    PERL("perl"),
    PHP("php"),
    POWERSHELL("powershell"),
    PRISMA("prisma"),
    PROTO_BUF("protobuf"),
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
    SYSTEM_VERILOG("systemverilog"),
    TCL("tcl"),
    TERRAFORM("terraform"),
    TLA_PLUS("tlaplus"),
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
        // Add aliases
        langs.put("text", PLAINTEXT);
        langs.put("plaintext", PLAINTEXT);
        langs.put("assembly", ASSEMBLY);
        langs.put("c++", CPP);
        langs.put("c#", C_SHARP);
        langs.put("docker", DOCKERFILE);
        langs.put("f#", F_SHARP);
        langs.put("ember", GLIMMER);
        langs.put("handlebars", GLIMMER);
        langs.put("jsx", JAVASCRIPT);
        langs.put("js", JAVASCRIPT);
        langs.put("ts", TYPESCRIPT);
        langs.put("objective-c", OBJ_C);
        langs.put("viml", VIM);
        langs.put("vimscript", VIM);
    }

    public static Lang fromString(String lang) {
        if (!langs.containsKey(lang)) {
            throw new IllegalArgumentException("Lang '" + lang + "' not found.");
        }
        return langs.get(lang);
    }
}
