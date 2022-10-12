/**
 * Returns true if the given error is a Wasm panic.
 */
 export function isWasmPanic(error: Error): boolean {
  return (error.name === 'RuntimeError')
}
