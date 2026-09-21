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

const [skill, index, card, catalog, config, homepage, robots] = await Promise.all([
  read(skillPath),
  readJson(indexPath),
  readJson(cardPath),
  readJson(catalogPath),
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

assert(card.serverInfo?.name === "docs", `${cardPath} has the wrong server name`);
assert(card.serverInfo?.version === "1.0.0", `${cardPath} has the wrong server version`);
assert(card.protocolVersion === "2025-06-18", `${cardPath} has the wrong protocol version`);
assert(
  card.transport?.type === "streamable-http" &&
    card.transport?.endpoint === "https://docs.lumis.sh/api/mcp",
  `${cardPath} does not advertise the deployed Streamable HTTP endpoint`,
);
assert(
  card.capabilities?.tools?.listChanged === true,
  `${cardPath} does not match the deployed tool capability`,
);

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

assert(
  homepage.includes('<link rel="ai-catalog" href="/.well-known/ai-catalog.json"'),
  "index.html does not advertise the AI Catalog",
);

const expectedHeaders = new Map([
  [
    "/.well-known/ai-catalog.json",
    { "content-type": "application/json; charset=utf-8", "access-control-allow-origin": "*" },
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
