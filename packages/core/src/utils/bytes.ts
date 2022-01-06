export function bufferToObject<T>(buffer: ArrayBuffer) {
  const message = new TextDecoder().decode(buffer)
  return JSON.parse(message) as T
}
