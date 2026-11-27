package sh.lumis;

import java.nio.ByteBuffer;
import java.nio.charset.CharacterCodingException;
import java.nio.charset.StandardCharsets;

public final class Highlighter {
    private static final Theme DEFAULT_THEME = Theme.GITHUB_DARK;
    private static final Lang DEFAULT_LANG = Lang.PLAINTEXT;
    private static final Formatter DEFAULT_FORMATTER = Formatter.TERMINAL;

    private final Lumis lumis;

    private final Theme theme;
    private final Lang lang;
    private final Formatter formatter;

    private Highlighter(Lumis lumis, Theme theme, Lang lang, Formatter formatter) {
        this.lumis = lumis;

        this.theme = theme;
        this.lang = lang;
        this.formatter = formatter;
    }

    public LumisResult highlight(String code) {
        return run(code.getBytes(StandardCharsets.UTF_8));
    }

    public LumisResult highlight(byte[] code) {
        // The module traps on invalid UTF-8 without freeing its argument buffers.
        try {
            StandardCharsets.UTF_8.newDecoder().decode(ByteBuffer.wrap(code));
        } catch (CharacterCodingException e) {
            return new LumisResult(e);
        }
        return run(code);
    }

    private LumisResult run(byte[] code) {
        return lumis.highlight(
                code,
                theme.value().getBytes(StandardCharsets.UTF_8),
                lang.value().getBytes(StandardCharsets.UTF_8),
                formatter.value());
    }

    public static final class Builder {
        private final Lumis lumis;

        private Theme theme;
        private Lang lang;
        private Formatter formatter;

        Builder(Lumis lumis) {
            this.lumis = lumis;
        }

        public Builder withTheme(String theme) {
            return withTheme(Theme.fromString(theme));
        }

        public Builder withTheme(Theme theme) {
            this.theme = theme;
            return this;
        }

        public Builder withLang(String lang) {
            return withLang(Lang.fromString(lang));
        }

        public Builder withLang(Lang lang) {
            this.lang = lang;
            return this;
        }

        public Builder withFormatter(Formatter formatter) {
            this.formatter = formatter;
            return this;
        }

        public Highlighter build() {
            Formatter selected = formatter != null ? formatter : DEFAULT_FORMATTER;
            if (theme != null) {
                switch (selected) {
                    case HTML_LINKED:
                    case BBCODE:
                        throw new IllegalArgumentException(
                                "Theme is not supported by the selected formatter");
                }
            }
            return new Highlighter(
                    lumis,
                    theme != null ? theme : DEFAULT_THEME,
                    lang != null ? lang : DEFAULT_LANG,
                    selected);
        }
    }
}
