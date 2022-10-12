import { exampleSchema } from './wasm/demo_serde_wasm'

function main() {
  console.info('Reading example schema from "demo-serde-wasm"...')
  const schema = exampleSchema()
  console.info('Schema read successfully:\n')

  console.log(JSON.stringify(schema, null, 2))
}

main()
