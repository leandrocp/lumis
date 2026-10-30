export const HERO_WOW_CODE = `<b><i class="c"># CLI</i>
<i class="p">$</i> <i class="f">curl</i> <i class="k">-LsSf</i> <i class="s">https://lumis.sh/install.sh</i> <i class="p">|</i> <i class="f">sh</i>
<i class="p">$</i> <i class="f">lumis</i> highlight app.js <i class="k">--theme</i> <i class="s">dracula</i></b><b><i class="c">// Rust</i>
<i class="k">use</i> lumis<i class="p">::{</i>highlight<i class="p">,</i> <i class="t">HtmlInlineBuilder</i><i class="p">};</i>
<i class="k">let</i> html <i class="p">=</i> <i class="f">highlight</i><i class="p">(</i><i class="s">&quot;const x = 1&quot;</i><i class="p">,</i> formatter<i class="p">);</i></b><b><i class="c"># Elixir</i>
<i class="t">Lumis</i><i class="p">.</i><i class="f">highlight!</i><i class="p">(</i><i class="s">&quot;const x = 1&quot;</i><i class="p">,</i>
  <i class="n">formatter</i><i class="p">:</i> <i class="p">{:</i>html_inline<i class="p">,</i> <i class="n">language</i><i class="p">:</i> <i class="s">&quot;javascript&quot;</i><i class="p">})</i></b><b><i class="c">// JavaScript / TypeScript</i>
<i class="k">import</i> <i class="p">{</i> highlight <i class="p">}</i> <i class="k">from</i> <i class="s">&#x27;@lumis-sh/lumis&#x27;</i>
<i class="k">const</i> html <i class="p">=</i> <i class="k">await</i> <i class="f">highlight</i><i class="p">(</i><i class="s">&#x27;const x = 1&#x27;</i><i class="p">,</i> options<i class="p">)</i></b><b><i class="c">// Browsers / CDN</i>
<i class="k">import</i> <i class="p">{</i> highlight <i class="p">}</i> <i class="k">from</i>
  <i class="s">&#x27;https://cdn.jsdelivr.net/npm/@lumis-sh/lumis/+esm&#x27;</i></b><b><i class="c">// Java (Lumis4J)</i>
<i class="k">var</i> lumis <i class="p">=</i> <i class="t">Lumis</i><i class="p">.</i><i class="f">builder</i><i class="p">().</i><i class="f">build</i><i class="p">();</i>
<i class="k">var</i> result <i class="p">=</i> lumis<i class="p">.</i><i class="f">highlight</i><i class="p">(</i><i class="s">&quot;const x = 1&quot;</i><i class="p">);</i></b><b><i class="c"># Python (fastpylight)</i>
<i class="k">from</i> fastpylight <i class="k">import</i> highlight_spans<i class="p">,</i> theme_css
html <i class="p">=</i> <i class="f">highlight_spans</i><i class="p">(</i><i class="s">&quot;const x = 1&quot;</i><i class="p">,</i> <i class="s">&quot;javascript&quot;</i><i class="p">)</i></b>`;
