export enum Mode {
  Patch = 'Patch',
  Config = 'Config',
}

export const activeTabs = Object.keys(Mode)

export type State = {
  mode: Mode,
  query: string,
  rewrite: string,
  config: string,
  source: string,
  lang: string,
}

// prefer old unicode hacks for backward compatibility
// https://base64.guru/developers/javascript/examples/unicode-strings
function utoa(data: string): string {
  return btoa(unescape(encodeURIComponent(data)))
}

function atou(base64: string): string {
  return decodeURIComponent(escape(atob(base64)))
}

export function serialize(state: State): string {
  return utoa(JSON.stringify(state))
}

export function deserialize(str: string): State {
  return JSON.parse(atou(str))
}

const source =
`// console.log() will be matched by pattern!
// click diff tab to see rewrite.

function tryAstGrep() {
  console.log('matched in metavar!')
}

const multiLineExpression =
  console
   .log('Also matched!')

if (true) {
  const notThis = 'console.log("not me")'
} else {
  console.debug('matched by YAML')
}`

const query = 'console.log($MATCH)'
const config = `
# YAML Rule is more powerful!
# https://ast-grep.github.io/guide/rule-config.html#rule
rule:
  any:
    - pattern: console.log($A)
    - pattern: console.debug($A)
fix:
  logger.log($A)
`.trim()

const defaultState = {
  mode: Mode.Patch,
  lang: 'javascript',
  query,
  rewrite: 'logger.log($MATCH)',
  config,
  source,
}

export function restoreState(): State {
  try {
    return {
      ...defaultState,
      ...deserialize(location.hash.slice(1)),
    }
  } catch {
    return defaultState
  }
}
