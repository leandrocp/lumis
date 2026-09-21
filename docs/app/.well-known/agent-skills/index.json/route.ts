const index = {
  $schema: "https://schemas.agentskills.io/discovery/0.2.0/schema.json",
  skills: [
    {
      name: "lumis-docs",
      type: "skill-md",
      description:
        "Find and read authoritative Lumis documentation through Markdown, search, or MCP.",
      url: "/.well-known/agent-skills/lumis-docs/SKILL.md",
      digest: "sha256:710bc87efa8eba225eabe443a333ab8947465ec0c407fe35022df8bd95580460",
    },
  ],
};

export const revalidate = false;

export function GET() {
  return Response.json(index, {
    headers: { "Access-Control-Allow-Origin": "*" },
  });
}
