package sh.lumis;

import java.nio.charset.StandardCharsets;
import java.util.List;
import run.endive.runtime.ImportValues;
import run.endive.runtime.Instance;
import run.endive.wasi.WasiOptions;
import run.endive.wasi.WasiPreview1;

public final class Lumis implements AutoCloseable {
    private final Instance instance;
    private final WasiPreview1 wasi;
    private final Lumis_ModuleExports exports;

    private Lumis() {
        var wasiOpts = WasiOptions.builder().inheritSystem().build();
        this.wasi = WasiPreview1.builder().withOptions(wasiOpts).build();
        var imports = ImportValues.builder().addFunction(wasi.toHostFunctions()).build();
        this.instance =
                Instance.builder(LumisModule.load())
                        .withImportValues(imports)
                        .withMachineFactory(LumisModule::create)
                        .build();
        this.exports = new Lumis_ModuleExports(this.instance);
    }

    public static Builder builder() {
        return new Builder();
    }

    public Highlighter.Builder highlighter() {
        return new Highlighter.Builder(this);
    }

    LumisResult highlight(byte[] code, byte[] theme, byte[] lang, int formatter) {
        try {
            var codePtr = exports.wasmMalloc(code.length);
            exports.memory().write(codePtr, code);

            var themePtr = exports.wasmMalloc(theme.length);
            exports.memory().write(themePtr, theme);

            var langPtr = exports.wasmMalloc(lang.length);
            exports.memory().write(langPtr, lang);

            var resultPtr =
                    exports.highlight(
                            codePtr,
                            code.length,
                            themePtr,
                            theme.length,
                            langPtr,
                            lang.length,
                            formatter);

            var result = unpackResult(resultPtr);
            return new LumisResult(result);
        } catch (RuntimeException e) {
            return new LumisResult(e);
        }
    }

    List<String> availableLanguages() {
        return lines(exports.availableLanguages());
    }

    List<String> availableThemes() {
        return lines(exports.availableThemes());
    }

    private List<String> lines(long packed) {
        return List.of(new String(unpackResult(packed), StandardCharsets.UTF_8).split("\n"));
    }

    private byte[] unpackResult(long packed) {
        var addr = (int) ((packed >>> 32) & 0xFFFFFFFFL);
        var len = (int) (packed & 0xFFFFFFFFL);
        var result = instance.memory().readBytes(addr, len);
        exports.wasmFree(addr);
        return result;
    }

    @Override
    public void close() {
        if (wasi != null) {
            wasi.close();
        }
    }

    public static final class Builder {
        private Builder() {}

        public Lumis build() {
            return new Lumis();
        }
    }
}
