package sh.lumis;

import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.approvaltests.Approvals;
import org.approvaltests.core.Options;
import org.junit.jupiter.api.Test;

/*
To update the results in approvals run with:
`APPROVAL_TESTS_USE_REPORTER=AutoApproveReporter`
*/
public class LumisTest {

    private static Options html() {
        return new Options().forFile().withExtension(".html");
    }

    private byte[] readCode(String fixture) throws Exception {
        return LumisTest.class.getResourceAsStream("/fixtures/" + fixture + "/code").readAllBytes();
    }

    // Poor man Sanity check: this should fail if we are not hitting the cache
    // without cache highlight takes seconds on the first iteration
    private long MAX_EXEC_TIME_MS = 500;

    private LumisResult highlight(Highlighter highlighter, byte[] code) {
        var before = System.nanoTime();
        var result = highlighter.highlight(code);
        var duration = (System.nanoTime() - before) / 1_000_000;

        // System.out.println("elapsed: " + duration);
        assertTrue(duration < MAX_EXEC_TIME_MS, "Highlighting took " + duration + " ms");
        return result;
    }

    @Test
    public void basicUsage() throws Exception {
        // Arrange
        var lumis = Lumis.builder().build().highlighter().withLang(Lang.JAVASCRIPT).build();
        var code = readCode("js");

        // Act
        var result = highlight(lumis, code);

        // Assert
        assertTrue(result.success());
        Approvals.verify(result.string());
    }

    @Test
    public void changeTheme() throws Exception {
        // Arrange
        var lumis =
                Lumis.builder()
                        .build()
                        .highlighter()
                        .withLang(Lang.JAVASCRIPT)
                        .withTheme(Theme.BAMBOO_LIGHT)
                        .build();
        var code = readCode("bamboo");

        // Act
        var result = highlight(lumis, code);

        // Assert
        assertTrue(result.success());
        Approvals.verify(result.string());
    }

    @Test
    public void changeLang() throws Exception {
        // Arrange
        var lumis =
                Lumis.builder()
                        .build()
                        .highlighter()
                        .withLang(Lang.RUST)
                        .withTheme(Theme.TOKYONIGHT_STORM)
                        .build();
        var code = readCode("rust");

        // Act
        var result = highlight(lumis, code);

        // Assert
        assertTrue(result.success());
        Approvals.verify(result.string());
    }

    @Test
    public void allDefaults() throws Exception {
        // Arrange
        var lumis = Lumis.builder().build().highlighter().build();
        var code = readCode("default");

        // Act
        var result = highlight(lumis, code);

        // Assert
        assertTrue(result.success());
        Approvals.verify(result.string());
    }

    @Test
    public void csv() throws Exception {
        // Arrange
        var lumis = Lumis.builder().build().highlighter().withLang("csv").build();
        var code = readCode("csv");

        // Act
        var result = highlight(lumis, code);

        // Assert
        assertTrue(result.success());
        Approvals.verify(result.string());
    }

    @Test
    public void java() throws Exception {
        // Arrange
        var lumis =
                Lumis.builder()
                        .build()
                        .highlighter()
                        .withLang(Lang.JAVA)
                        .withTheme(Theme.AURA_SOFT_DARK)
                        .build();
        var code = readCode("java");

        // Act
        var result = highlight(lumis, code);

        // Assert
        assertTrue(result.success());
        Approvals.verify(result.string());
    }

    @Test
    public void inlineHtmlFormatter() throws Exception {
        // Arrange
        var lumis =
                Lumis.builder()
                        .build()
                        .highlighter()
                        .withLang(Lang.JAVA)
                        .withTheme(Theme.CATPPUCCIN_FRAPPE)
                        .withFormatter(Formatter.HTML_INLINE)
                        .build();
        var code = readCode("java");

        // Act
        var result = highlight(lumis, code);

        // Assert
        assertTrue(result.success());
        Approvals.verify(result.string(), html());
    }

    @Test
    public void linkedHtmlFormatter() throws Exception {
        // Arrange
        var lumis =
                Lumis.builder()
                        .build()
                        .highlighter()
                        .withLang(Lang.RUST)
                        .withFormatter(Formatter.HTML_LINKED)
                        .build();
        var code = readCode("rust");

        // Act
        var result = highlight(lumis, code);

        // Assert
        assertTrue(result.success());
        Approvals.verify(result.string(), html());
    }

    @Test
    public void bbcodeFormatter() throws Exception {
        // Arrange
        var lumis =
                Lumis.builder()
                        .build()
                        .highlighter()
                        .withLang(Lang.RUST)
                        .withFormatter(Formatter.BBCODE)
                        .build();
        var code = readCode("rust");

        // Act
        var result = highlight(lumis, code);

        // Assert
        assertTrue(result.success());
        Approvals.verify(result.string());
    }

    @Test
    public void verifyKotlinLanguage() throws Exception {
        // Arrange
        var lumis = Lumis.builder().build().highlighter().withLang(Lang.KOTLIN).build();
        var code = readCode("kotlin");

        // Act
        var result = highlight(lumis, code);

        // Assert
        assertTrue(result.success());
        Approvals.verify(result.string());
    }

    @Test
    public void wat() throws Exception {
        // Arrange
        var lumis = Lumis.builder().build().highlighter().withLang(Lang.WAT).build();
        var code = readCode("wat");

        // Act
        var result = highlight(lumis, code);

        // Assert
        assertTrue(result.success());
        Approvals.verify(result.string());
    }

    @Test
    public void bbcodeFormatterRejectsTheme() {
        // Wrong condition: BBCODE with theme should throw
        var builder =
                Lumis.builder()
                        .build()
                        .highlighter()
                        .withLang(Lang.RUST)
                        .withTheme(Theme.CATPPUCCIN_FRAPPE)
                        .withFormatter(Formatter.BBCODE);

        assertThrows(IllegalArgumentException.class, builder::build);
    }

    @Test
    public void bbcodeFormatterWithoutTheme() throws Exception {
        // Right condition: BBCODE without theme should work
        var lumis =
                Lumis.builder()
                        .build()
                        .highlighter()
                        .withLang(Lang.RUST)
                        .withFormatter(Formatter.BBCODE)
                        .build();
        var code = readCode("rust");

        // Act
        var result = highlight(lumis, code);

        // Assert
        assertTrue(result.success());
        Approvals.verify(result.string());
    }
}
