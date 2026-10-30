import WebGLFluidEnhanced from "webgl-fluid-enhanced";

const COLOR_PALETTE = ["#8C6BFF", "#FF5FAE", "#FF6F6F", "#FF7A45", "#FFC23D"];

export function mountHeroFluid(container: HTMLElement): WebGLFluidEnhanced {
  const fluid = new WebGLFluidEnhanced(container);

  fluid.setConfig({
    transparent: true,
    colorPalette: COLOR_PALETTE,
    colorful: false,
    colorUpdateSpeed: 4,
    densityDissipation: 0.965,
    velocityDissipation: 0.992,
    pressure: 0.8,
    curl: 8,
    splatRadius: 0.25,
    splatForce: 1200,
    shading: false,
    bloom: true,
    bloomIntensity: 0.08,
    bloomThreshold: 0.35,
    bloomSoftKnee: 0.7,
    sunrays: false,
    hover: true,
    brightness: 0.35,
    simResolution: 128,
    dyeResolution: 1024,
  });

  fluid.start();

  return fluid;
}
