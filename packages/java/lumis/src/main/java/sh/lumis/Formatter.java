package sh.lumis;

public enum Formatter {
    // avoiding using zero as it's null in C
    TERMINAL(1),
    HTML_INLINE(2),
    // Linked doesn't support "themes"
    HTML_LINKED(3),
    BBCODE(4);

    private final int value;

    Formatter(int value) {
        this.value = value;
    }

    public int value() {
        return value;
    }
}
