export async function GET(request) {
  const url = new URL(request.url);
  const version = url.searchParams.get("version");
  const format = url.searchParams.get("format");

  if (!version || !/^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$/.test(version)) {
    return new Response("Invalid Lumis version\n", { status: 400 });
  }

  if (format !== "sh" && format !== "ps1") {
    return new Response("Invalid installer format\n", { status: 400 });
  }

  const installerResponse = await fetch(new URL(`/install.${format}`, url));
  if (!installerResponse.ok) {
    return new Response("Installer unavailable\n", { status: 502 });
  }

  const installer = await installerResponse.text();
  const body =
    format === "sh"
      ? installer.replace(/^(#![^\n]*\n)/, `$1LUMIS_VERSION='${version}'\nexport LUMIS_VERSION\n`)
      : `$env:LUMIS_VERSION = '${version}'\n${installer}`;

  return new Response(body, {
    headers: {
      "Cache-Control": "public, max-age=300, s-maxage=86400",
      "Content-Type": "text/plain; charset=utf-8",
    },
  });
}
