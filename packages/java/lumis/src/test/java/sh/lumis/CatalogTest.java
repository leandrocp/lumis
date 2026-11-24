package sh.lumis;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.Arrays;
import java.util.TreeSet;
import java.util.stream.Collectors;
import org.junit.jupiter.api.Test;

public class CatalogTest {

    @Test
    public void langEnumMatchesModule() {
        try (var lumis = Lumis.builder().build()) {
            var module = new TreeSet<>(lumis.availableLanguages());
            module.remove("plaintext");
            var declared =
                    Arrays.stream(Lang.values())
                            .filter(lang -> lang != Lang.PLAINTEXT)
                            .map(Lang::value)
                            .collect(Collectors.toCollection(TreeSet::new));

            assertEquals(module, declared);
        }
    }

    @Test
    public void themeEnumMatchesModule() {
        try (var lumis = Lumis.builder().build()) {
            var module = new TreeSet<>(lumis.availableThemes());
            var declared =
                    Arrays.stream(Theme.values())
                            .map(Theme::value)
                            .collect(Collectors.toCollection(TreeSet::new));

            assertEquals(module, declared);
        }
    }
}
