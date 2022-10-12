import { parseSchema, validateAST } from './wasm/schema_parser_wasm'
import type { SchemaAST } from './wasm/schema_parser_wasm'
import { displayError } from './utils/displayError'

async function main() {
  await validateSuccess()
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

async function validateFailure() {
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
