import { createRoot } from "react-dom/client";
import { createHighlighter } from "@lumis-sh/lumis";
import { CodeBlock } from "@lumis-sh/react";
import { htmlInline, htmlMultiThemes } from "@lumis-sh/lumis/formatters";
import githubDark from "@lumis-sh/themes/github_dark";
import githubLight from "@lumis-sh/themes/github_light";
import web from "@lumis-sh/wasm-bundle-web";

const highlighter = await createHighlighter({ languages: [web] });
await Promise.all([highlighter.loadLanguage("javascript"), highlighter.loadLanguage("tsx")]);

const mainStyle = { fontFamily: "system-ui, sans-serif", margin: "2rem auto", maxWidth: 880 };
const spacerStyle = { height: 24 };
const javascriptFormatter = htmlInline({ language: "javascript", theme: githubLight });
const typescriptFormatter = htmlMultiThemes({
  language: "tsx",
  themes: { light: githubLight, dark: githubDark },
  defaultTheme: "light-dark()",
});

function App() {
  return (
    <main style={mainStyle}>
      <h1>Lumis React</h1>

      <CodeBlock highlighter={highlighter} formatter={javascriptFormatter}>
        {`export function greet(name) {
  return \`Hello, \${name}!\`
}`}
      </CodeBlock>

      <div style={spacerStyle} />

      <CodeBlock highlighter={highlighter} formatter={typescriptFormatter}>
        {`export function Button() {
  return <button type="button">Click me</button>
}`}
      </CodeBlock>
    </main>
  );
}

createRoot(document.querySelector("#app")).render(<App />);
