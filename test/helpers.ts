import { performance } from 'perf_hooks';

export async function converge<T>(fn: () => T, timeout = 2000): Promise<T> {
  let startTime = performance.now();
  while(true) {
    try {
      return fn();
    } catch(e) {
      let diff = performance.now() - startTime;
      if(diff > timeout) {
        throw e;
      } else {
        await new Promise((resolve) => setTimeout(resolve, 1));
      }
    }
  }
}
