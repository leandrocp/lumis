import assert from "node:assert/strict";
import { createServer } from "vite";

const server = await createServer({
  server: { middlewareMode: true, watch: null, hmr: false, ws: false },
  optimizeDeps: { noDiscovery: true, include: [] },
  appType: "custom",
});

try {
  const { THEMES, AVAILABLE_THEMES, loadTheme } = await server.ssrLoadModule("/src/data/themes.ts");
  assert.equal(THEMES.length, 256);
  assert.equal(AVAILABLE_THEMES.length, 252);
  assert.deepEqual(
    THEMES.filter((theme) => !theme.available)
      .map((theme) => theme.id)
      .sort(),
    ["darkplus", "no_clown_fiesta", "poimandres", "yorumi"],
  );
  assert.ok(AVAILABLE_THEMES.some((theme) => theme.pack === "copyleft"));
  assert.ok(AVAILABLE_THEMES.some((theme) => theme.pack === "default"));
  for (const theme of AVAILABLE_THEMES) {
    const data = await loadTheme(theme.id);
    assert.equal(data.name, theme.id);
    assert.equal(data.appearance, theme.appearance);
    assert.ok(Object.keys(data.highlights).length > 0);
  }
  for (const id of ["darkplus", "no_clown_fiesta", "poimandres", "yorumi"]) {
    await assert.rejects(loadTheme(id), /unavailable for redistribution/u);
  }
  await assert.rejects(loadTheme("not_a_theme"), /Unknown theme/u);
  console.log("Theme demos: 252 loadable payloads, four metadata-only entries rejected.");
} finally {
  await server.close();
}
