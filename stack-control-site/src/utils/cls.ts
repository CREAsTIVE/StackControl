export function cls(...params: (string | undefined)[]) {
  return params.filter(e => e).join(" ")
}