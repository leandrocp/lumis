import { HERO_WOW_CODE } from "./hero-wow-code";

declare global {
  interface Navigator {
    connection?: { saveData?: boolean };
  }
}

const CFG = {
  follow: 0.16,
  ambientFollow: 0.05,
  ambientPeriodX: 23_000,
  ambientPeriodY: 17_000,
  ambientOpacity: 0.72,
  settleDelay: 1200,
  touchHold: 3000,
  fadeMs: 320,
  slowFrameMs: 28,
  slowFrameCount: 45,
  static: { x: 0.79, y: 0.3, xs: 0.5, ys: 0.84 },
};

export function mountHeroWow(host: HTMLElement) {
  const section = host.closest("section");
  if (!section) return;
  const blocks = HERO_WOW_CODE + HERO_WOW_CODE;
  host.innerHTML = `<div class="wow-field"><pre class="wow-code wow-base">${blocks}</pre><div class="wow-glow"></div><div class="wow-lens"><pre class="wow-code wow-lit">${blocks}</pre></div></div>`;
  const glow = host.querySelector<HTMLElement>(".wow-glow")!;
  const lens = host.querySelector<HTMLElement>(".wow-lens")!;
  const lit = host.querySelector<HTMLElement>(".wow-lit")!;
  const base = host.querySelector<HTMLElement>(".wow-base")!;
  const reducedMotion = matchMedia("(prefers-reduced-motion: reduce)");
  const lowPower = () =>
    reducedMotion.matches ||
    navigator.connection?.saveData === true ||
    (navigator.hardwareConcurrency || 8) <= 2;
  let width = 0,
    height = 0,
    left = 0,
    top = 0,
    radius = 0,
    mode = "ambient",
    visible = true,
    raf = 0,
    settleAt = 0,
    origin = performance.now(),
    last = origin,
    frames = 0,
    slowFrames = 0;
  const state = {
    x: 0,
    y: 0,
    targetX: 0,
    targetY: 0,
    opacity: 0,
    targetOpacity: CFG.ambientOpacity,
    scale: 0.85,
  };
  const measure = () => {
    const rect = host.getBoundingClientRect();
    width = rect.width;
    height = rect.height;
    left = rect.left;
    top = rect.top + scrollY;
    radius = lens.offsetWidth / 2;
    for (const element of [base, lit]) {
      element.style.width = `${width}px`;
      element.style.height = `${height}px`;
    }
  };
  const apply = () => {
    const transform = `translate3d(${state.x}px,${state.y}px,0)`;
    glow.style.transform = `${transform} scale(${state.scale})`;
    lens.style.transform = `${transform} scale(${state.scale})`;
    lit.style.transform = `translate3d(${radius - state.x}px,${radius - state.y}px,0) scale(${1 / state.scale})`;
    glow.style.opacity = `${state.opacity}`;
    lens.style.opacity = `${state.opacity}`;
  };
  const park = () => {
    const small = width < 640;
    state.x = state.targetX = width * (small ? CFG.static.xs : CFG.static.x);
    state.y = state.targetY = height * (small ? CFG.static.ys : CFG.static.y);
    state.opacity = state.targetOpacity = state.scale = 1;
    apply();
  };
  const stop = () => {
    cancelAnimationFrame(raf);
    raf = 0;
  };
  const toStatic = () => {
    mode = "static";
    stop();
    park();
  };
  const phase = (x: number) =>
    (Math.asin(Math.max(-1, Math.min(1, (x / width - 0.5) / 0.36))) / (2 * Math.PI)) *
    CFG.ambientPeriodX;
  const updateTarget = (now: number) => {
    if (mode === "ambient") {
      const t = now - origin;
      state.targetX = width * (0.5 + 0.36 * Math.sin((t * 2 * Math.PI) / CFG.ambientPeriodX));
      state.targetY =
        height * (0.52 + 0.24 * Math.sin((t * 2 * Math.PI) / CFG.ambientPeriodY + 1.1));
      state.targetOpacity = CFG.ambientOpacity;
    }
    if ((mode === "settle" || mode === "touch") && now > settleAt) {
      mode = "ambient";
      origin = now - phase(state.x);
    }
  };
  const isSlow = (dt: number) =>
    frames++ < 90 && dt > CFG.slowFrameMs && ++slowFrames >= CFG.slowFrameCount;
  const isMoving = () =>
    mode !== "track" ||
    Math.abs(state.targetX - state.x) + Math.abs(state.targetY - state.y) > 0.3 ||
    Math.abs(state.targetOpacity - state.opacity) > 0.005 ||
    Math.abs(1 - state.scale) > 0.002;
  const step = (now: number) => {
    const dt = Math.min(64, now - last);
    last = now;
    updateTarget(now);
    const factor =
      1 - Math.pow(1 - (mode === "ambient" ? CFG.ambientFollow : CFG.follow), dt / 16.67);
    state.x += (state.targetX - state.x) * factor;
    state.y += (state.targetY - state.y) * factor;
    state.opacity += (state.targetOpacity - state.opacity) * Math.min(1, (dt / CFG.fadeMs) * 3);
    state.scale += (1 - state.scale) * Math.min(1, (dt / CFG.fadeMs) * 3);
    apply();
    if (isSlow(dt)) {
      toStatic();
      return false;
    }
    return isMoving();
  };
  const loop = (now: number) => {
    raf = 0;
    if (step(now) && visible && !document.hidden) raf = requestAnimationFrame(loop);
  };
  const kick = () => {
    if (!raf && mode !== "static" && visible && !document.hidden) {
      last = performance.now();
      raf = requestAnimationFrame(loop);
    }
  };
  const local = (event: PointerEvent) => [event.clientX - left, event.clientY - (top - scrollY)];
  const track = (event: PointerEvent) => {
    if (mode === "static" || (event.pointerType === "touch" && mode !== "touch")) return;
    [state.targetX, state.targetY] = local(event);
    state.targetOpacity = 1;
    if (event.pointerType === "touch") settleAt = performance.now() + CFG.touchHold;
    else mode = "track";
    kick();
  };
  const settle = () => {
    if (mode === "static") return;
    mode = "settle";
    state.targetOpacity = CFG.ambientOpacity;
    settleAt = performance.now() + CFG.settleDelay;
    kick();
  };
  const focus = (event: FocusEvent) => {
    const target = event.target;
    if (!(target instanceof HTMLElement) || !target.matches(":focus-visible")) return;
    const rect = target.getBoundingClientRect();
    state.targetX = rect.left - left + rect.width / 2;
    state.targetY = rect.top + scrollY - top + rect.height / 2;
    state.targetOpacity = 1;
    mode = "track";
    kick();
  };
  const refreshTop = () => {
    top = host.getBoundingClientRect().top + scrollY;
  };
  new ResizeObserver(() => {
    measure();
    if (mode === "static") park();
    else kick();
  }).observe(host);
  new IntersectionObserver(([entry]) => {
    visible = entry.isIntersecting;
    if (visible) kick();
    else stop();
  }).observe(host);
  section.addEventListener("pointermove", track, { passive: true });
  section.addEventListener("pointerenter", track, { passive: true });
  section.addEventListener("pointerleave", settle, { passive: true });
  section.addEventListener(
    "pointerdown",
    (event) => {
      if (event.pointerType === "touch") {
        mode = "touch";
        state.scale = 0.9;
        track(event);
      }
    },
    { passive: true },
  );
  section.addEventListener("pointercancel", settle, { passive: true });
  section.addEventListener("focusin", focus, { passive: true });
  section.addEventListener("focusout", settle, { passive: true });
  addEventListener("scroll", refreshTop, { passive: true });
  addEventListener("visibilitychange", () => {
    if (document.hidden) stop();
    else kick();
  });
  reducedMotion.addEventListener("change", () => {
    if (lowPower()) toStatic();
    else {
      mode = "ambient";
      origin = performance.now();
      kick();
    }
  });
  measure();
  if (lowPower()) toStatic();
  else {
    state.x = state.targetX = width / 2;
    state.y = state.targetY = height / 2;
    host.classList.add("is-on");
    kick();
  }
  host.classList.add("is-on");
}
