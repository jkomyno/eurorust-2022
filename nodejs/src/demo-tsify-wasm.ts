import { exampleSchema } from './wasm/demo_tsify_wasm'

function main() {
  console.info('Reading example schema from "demo-tsify-wasm"...')
  const schema = exampleSchema()
  console.info('Schema read successfully:\n')

  console.log(JSON.stringify(schema, null, 2))
}

main()
