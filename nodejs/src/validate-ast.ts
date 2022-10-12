import { match } from 'ts-pattern'
import { parseSchema, validateAST } from './wasm/schema_parser_wasm'
import type { SchemaAST } from './wasm/schema_parser_wasm'
import { displayError } from './utils/displayError'

async function main() {
  // @ts-ignore
  await match(process.argv[2])
    .with('success', validateSuccess)
    .with('error', validateError)
    .with('panic', validatePanic)
    .otherwise(() => {
      console.error('Please specify a valid demo: success, error, panic')
    })
}

main()
  .catch(displayError)

async function validateSuccess() {
  const schema = /* prisma */ `

    datasource db {
      provider = "sqlite"
      url = "file:./dev.db"
    }
  
  `

  const ast: SchemaAST = parseSchema(schema)
  console.info('Validating AST...\n')

  validateAST(ast)

  console.info('AST validated successfully!')
}

// This throws an error because:
// - more than one datasource is provided
// - the cockroachdb provider is not supported
// - the url for postgres doesn't start with the "postgres://" protocol
async function validateError() {
  const schema = /* prisma */ `

    datasource db {
      provider = "cockroachdb"
      url = "postgres://jkomyno:prisma@localhost:5432"
    }

    datasource db {
      provider = "postgres"
      url = "mysql://jkomyno:prisma@localhost:5432"
    }
  
  `

  const ast: SchemaAST = parseSchema(schema)
  console.info('Validating AST...\n')

  validateAST(ast)

  console.info('AST validated successfully!')
}

// This panics because:
// - env urls are not supported
async function validatePanic() {
  const schema = /* prisma */ `

    datasource db {
      provider = "cockroachdb"
      url = env("DATABASE_URL")
    }
  
  `

  const ast: SchemaAST = parseSchema(schema)
  console.info('Validating AST...\n')

  validateAST(ast)

  console.info('AST validated successfully!')
}
