import { isWasmPanic } from './isWasmPanic'

export function displayError(e: unknown) {
  const error = e as Error

  if (isWasmPanic(error)) {
    console.log('[node:panic]', error)
  } else {
    const errorMessageAsJSON = JSON.parse(error.message)
    console.log('[node:error]', errorMessageAsJSON)
  }
}
