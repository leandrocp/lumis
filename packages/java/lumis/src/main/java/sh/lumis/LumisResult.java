package sh.lumis;

import java.nio.charset.StandardCharsets;

public final class LumisResult {
    private final boolean success;
    private final byte[] result;
    private final Throwable error;

    LumisResult(byte[] result) {
        this.success = true;
        this.result = result;
        this.error = null;
    }

    LumisResult(Throwable error) {
        this.success = false;
        this.result = null;
        this.error = error;
    }

    public byte[] bytes() {
        return result;
    }

    public String string() {
        return new String(result, StandardCharsets.UTF_8);
    }

    public boolean success() {
        return success;
    }

    public boolean failure() {
        return !success;
    }

    public Throwable error() {
        return error;
    }
}
