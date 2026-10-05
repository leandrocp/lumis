import { measure } from "mitata";

// Keep parsing the numeric prefix: `parseInt("10samples", 10)` historically
// accepts it, while `Number("10samples")` would reject the value.
// oxlint-disable-next-line unicorn/prefer-number-coercion -- preserve partial numeric parsing.
const minimumSamples = Number.parseInt(process.env.BENCH_SAMPLES ?? "10", 10);
// oxlint-disable-next-line unicorn/prefer-number-coercion -- preserve partial numeric parsing.
const measurementSeconds = Number.parseFloat(process.env.BENCH_TIME_SECONDS ?? "1");

if (!Number.isSafeInteger(minimumSamples) || minimumSamples < 2) {
  throw new Error("BENCH_SAMPLES must be at least two");
}
if (!Number.isFinite(measurementSeconds) || measurementSeconds <= 0) {
  throw new Error("BENCH_TIME_SECONDS must be positive");
}

const retained = { value: undefined };
const release = () => {
  retained.value = undefined;
  globalThis.gc?.();
};

/**
 * Times `call` the way every JavaScript row is timed, in the scenario suite and
 * in the visual comparison alike, so the two cannot drift apart.
 *
 * The result is kept until the next sample so the call cannot be optimised away,
 * and the heap is collected between samples so one sample does not pay for the
 * garbage of the last. mitata awaits `call` only if it returns a promise.
 */
export async function measureCall(call) {
  const stats = await measure(() => (retained.value = call()), {
    min_samples: minimumSamples,
    max_samples: 1_000_000,
    min_cpu_time: measurementSeconds * 1e9,
    warmup_samples: 1,
    batch_samples: 1,
    gc: release,
    inner_gc: true,
  });
  release();
  return stats;
}
