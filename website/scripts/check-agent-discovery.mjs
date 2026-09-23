import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";

const websiteRoot = new URL("../", import.meta.url);

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

async function read(path) {
  return readFile(new URL(path, websiteRoot), "utf8");
}

async function readJson(path) {
  return JSON.parse(await read(path));
}

function headerMap(config, source) {
  const entry = config.headers?.find((candidate) => candidate.source === source);
  return new Map(entry?.headers?.map(({ key, value }) => [key.toLowerCase(), value]) ?? []);
}

const skillPath = "public/.well-known/agent-skills/lumis/SKILL.md";
const indexPath = "public/.well-known/agent-skills/index.json";
const cardPath = "public/.well-known/mcp/server-card.json";
const catalogPath = "public/.well-known/ai-catalog.json";
const apiCatalogPath = "public/.well-known/api-catalog";
const openapiPath = "public/openapi/versioned-installer.json";

// The file that serves a lumis.sh URL: a Vercel function under api/, or a static asset.
function sourceFile({ pathname }) {
  return pathname.startsWith("/api/") ? `${pathname.slice(1)}.js` : `public${pathname}`;
}

const [skill, index, card, catalog, apiCatalog, openapi, installer, config, homepage, robots] =
  await Promise.all([
    read(skillPath),
    readJson(indexPath),
    readJson(cardPath),
    readJson(catalogPath),
    readJson(apiCatalogPath),
    readJson(openapiPath),
    read("api/versioned-installer.js"),
    readJson("vercel.json"),
    read("index.html"),
    read("public/robots.txt"),
  ]);

const contentSignal = "Content-Signal: ai-train=yes, search=yes, ai-input=yes";
const robotsGroups = robots.trim().split(/\n\n+/);
const userAgentGroups = robotsGroups.filter((group) => group.startsWith("User-agent:"));
assert(userAgentGroups.length > 0, "robots.txt has no user-agent groups");
for (const group of userAgentGroups) {
  assert(group.includes(contentSignal), `robots.txt group lacks ${contentSignal}:\n${group}`);
}
assert(
  robots.includes("Agentmap: https://lumis.sh/.well-known/ai-catalog.json"),
  "robots.txt does not advertise the AI Catalog",
);

assert(
  index.$schema === "https://schemas.agentskills.io/discovery/0.2.0/schema.json",
  `${indexPath} has the wrong schema`,
);
assert(Array.isArray(index.skills) && index.skills.length > 0, `${indexPath} has no skills`);

const skillEntry = index.skills.find(({ name }) => name === "lumis");
assert(skillEntry, `${indexPath} does not list the Lumis skill`);
assert(skillEntry.type === "skill-md", "The Lumis skill must use the skill-md distribution");
assert(
  skillEntry.url === "/.well-known/agent-skills/lumis/SKILL.md",
  "The Lumis skill URL is not canonical",
);

const skillDigest = createHash("sha256").update(skill).digest("hex");
assert(
  skillEntry.digest === `sha256:${skillDigest}`,
  `${indexPath} has a stale Lumis skill digest; expected sha256:${skillDigest}`,
);

const frontmatter = skill.match(/^---\n(?<fields>[\s\S]*?)\n---\n/);
assert(frontmatter?.groups, `${skillPath} has no YAML frontmatter`);
const name = frontmatter.groups.fields.match(/^name: (?<value>.+)$/m)?.groups?.value;
const description = frontmatter.groups.fields.match(/^description: (?<value>.+)$/m)?.groups?.value;
assert(name === skillEntry.name, `${skillPath} name does not match ${indexPath}`);
assert(
  description === skillEntry.description,
  `${skillPath} description does not match ${indexPath}`,
);

assert(
  card.$schema === "https://static.modelcontextprotocol.io/schemas/v1/server-card.schema.json",
  `${cardPath} has the wrong schema`,
);
assert(card.name === "sh.lumis/docs", `${cardPath} has the wrong server name`);
assert(card.version === "1.0.0", `${cardPath} has the wrong server version`);
assert(
  card.remotes?.some(
    ({ type, url, supportedProtocolVersions }) =>
      type === "streamable-http" &&
      url === "https://docs.lumis.sh/api/mcp" &&
      supportedProtocolVersions?.includes("2025-06-18"),
  ),
  `${cardPath} does not advertise the deployed Streamable HTTP endpoint and protocol`,
);
assert(!("capabilities" in card), `${cardPath} must leave capabilities to MCP negotiation`);

assert(catalog.specVersion === "1.0", `${catalogPath} has the wrong specVersion`);
assert(catalog.host?.displayName === "Lumis", `${catalogPath} has no Lumis host`);
assert(Array.isArray(catalog.entries) && catalog.entries.length > 0, `${catalogPath} is empty`);

for (const entry of catalog.entries) {
  assert(
    entry.identifier.startsWith("urn:air:lumis.sh:"),
    `Invalid ARD identifier: ${entry.identifier}`,
  );
  assert(typeof entry.displayName === "string", `${entry.identifier} has no displayName`);
  assert(
    typeof entry.type === "string" && entry.type.includes("/"),
    `${entry.identifier} has no media type`,
  );
  assert(
    "url" in entry !== "data" in entry,
    `${entry.identifier} must contain exactly one of url or data`,
  );
  assert(
    Array.isArray(entry.representativeQueries) &&
      entry.representativeQueries.length >= 2 &&
      entry.representativeQueries.length <= 5,
    `${entry.identifier} must contain 2-5 representative queries`,
  );
}

// RFC 9727 API catalog: a linkset whose anchors are APIs and whose links are RFC 8631 relations.
assert(
  Array.isArray(apiCatalog.linkset) && apiCatalog.linkset.length > 0,
  `${apiCatalogPath} has no linkset`,
);

const anchors = new Set();
for (const entry of apiCatalog.linkset) {
  const anchor = URL.parse(entry.anchor);
  assert(anchor?.protocol === "https:", `Invalid API catalog anchor: ${entry.anchor}`);
  assert(!anchors.has(entry.anchor), `${entry.anchor} is listed twice in ${apiCatalogPath}`);
  anchors.add(entry.anchor);

  for (const relation of ["service-desc", "service-doc", "status"]) {
    const links = entry[relation];
    assert(
      Array.isArray(links) && links.length > 0,
      `${entry.anchor} has no ${relation} link; agents need all three to use the API`,
    );
    for (const link of links) {
      const target = URL.parse(link.href);
      assert(target?.protocol === "https:", `Invalid ${relation} href for ${entry.anchor}`);
      assert(
        typeof link.type === "string" && link.type.includes("/"),
        `${link.href} has no media type`,
      );
      if (target.host !== "lumis.sh") continue;
      await read(sourceFile(target)).catch(() => {
        throw new Error(
          `${link.href} is in ${apiCatalogPath} but ${sourceFile(target)} is missing`,
        );
      });
    }
  }
}

assert(
  card.remotes.every(({ url }) => anchors.has(url)),
  `${apiCatalogPath} does not list every MCP endpoint advertised by ${cardPath}`,
);

const installerAnchor = "https://lumis.sh/api/versioned-installer";
assert(anchors.has(installerAnchor), `${apiCatalogPath} does not list the installer API`);
await read(sourceFile(new URL(installerAnchor))).catch(() => {
  throw new Error(`${installerAnchor} is in ${apiCatalogPath} but its function is missing`);
});

assert(openapi.openapi?.startsWith("3.1"), `${openapiPath} is not an OpenAPI 3.1 description`);
assert(
  openapi.servers?.[0]?.url === "https://lumis.sh",
  `${openapiPath} does not describe lumis.sh`,
);
const described = openapi.paths?.["/api/versioned-installer"]?.get;
assert(described, `${openapiPath} does not describe GET /api/versioned-installer`);

// The published version pattern has to stay in step with the one the function enforces.
const enforced = installer.match(/\/(?<pattern>\^[^/]+\$)\/\.test\(version\)/)?.groups?.pattern;
assert(enforced, "api/versioned-installer.js no longer validates version with a literal pattern");
const documented = described.parameters?.find((parameter) => parameter.name === "version")?.schema
  ?.pattern;
assert(
  documented === enforced,
  `${openapiPath} documents version as ${documented}; the API enforces ${enforced}`,
);

assert(
  homepage.includes('rel="ai-catalog"') &&
    homepage.includes('href="/.well-known/ai-catalog.json"') &&
    homepage.includes('type="application/ai-catalog+json"'),
  "index.html does not advertise the AI Catalog",
);

assert(
  homepage.includes('rel="api-catalog"') &&
    homepage.includes('href="/.well-known/api-catalog"') &&
    homepage.includes('type="application/linkset+json"'),
  "index.html does not advertise the API Catalog",
);

const apiCatalogLink = '</.well-known/api-catalog>; rel="api-catalog"';
const homepageLink = headerMap(config, "/").get("link");
assert(homepageLink?.includes(apiCatalogLink), `/ must advertise ${apiCatalogLink}`);
const markdownLink = config.routes?.find(({ src }) => src === "^/$")?.headers?.Link;
assert(
  markdownLink?.includes(apiCatalogLink),
  `The Markdown route for / must advertise ${apiCatalogLink}`,
);

const expectedHeaders = new Map([
  [
    "/.well-known/api-catalog",
    {
      "content-type": 'application/linkset+json; profile="https://www.rfc-editor.org/info/rfc9727"',
      "access-control-allow-origin": "*",
    },
  ],
  [
    "/openapi/versioned-installer.json",
    { "content-type": "application/json; charset=utf-8", "access-control-allow-origin": "*" },
  ],
  [
    "/.well-known/ai-catalog.json",
    {
      "content-type": "application/ai-catalog+json; charset=utf-8",
      "access-control-allow-origin": "*",
    },
  ],
  [
    "/.well-known/mcp/server-card.json",
    {
      "content-type": "application/mcp-server-card+json; charset=utf-8",
      "access-control-allow-origin": "*",
    },
  ],
  [
    "/.well-known/agent-skills/index.json",
    { "content-type": "application/json; charset=utf-8", "access-control-allow-origin": "*" },
  ],
  [
    "/.well-known/agent-skills/lumis/SKILL.md",
    { "content-type": "text/markdown; charset=utf-8", "access-control-allow-origin": "*" },
  ],
]);

for (const [source, headers] of expectedHeaders) {
  const configured = headerMap(config, source);
  for (const [key, value] of Object.entries(headers)) {
    assert(configured.get(key) === value, `${source} must set ${key}: ${value}`);
  }
}
