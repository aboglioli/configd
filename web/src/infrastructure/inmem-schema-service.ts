import { Schema, SchemaService } from 'domain/schema';
import { PropKind } from 'domain/prop';

const schemas: Schema[] = [
  {
    id: 'schema-1',
    name: 'Schema 1',
    schema: {
      env: {
        $schema: {
          kind: PropKind.String,
          required: true,
          allowed_values: ['dev', 'stg', 'prod'],
        },
      },
    },
    configs: [
      {
        id: 'dev',
        name: 'Dev',
        valid: true,
        checksum: 'abcd1234',
        created_at: new Date(),
        updated_at: new Date(),
        version: 2,
      },
      {
        id: 'stg',
        name: 'Staging',
        valid: true,
        checksum: 'qwerty1234',
        created_at: new Date(),
        updated_at: new Date(),
        version: 1,
      },
    ],
    created_at: new Date(),
    updated_at: new Date(),
    version: 1,
  },
  {
    id: 'schema-2',
    name: 'Schema 2',
    schema: {
      env: {
        $schema: {
          kind: PropKind.String,
          required: true,
          allowed_values: ['dev', 'stg', 'prod'],
        },
      },
    },
    configs: [
      {
        id: 'dev',
        name: 'Dev',
        valid: true,
        checksum: 'abcd1234',
        created_at: new Date(),
        updated_at: new Date(),
        version: 2,
      },
      {
        id: 'stg',
        name: 'Staging',
        valid: true,
        checksum: 'qwerty1234',
        created_at: new Date(),
        updated_at: new Date(),
        version: 1,
      },
    ],
    created_at: new Date(),
    updated_at: new Date(),
    version: 1,
  },
];

export class InMemSchemaService implements SchemaService {
  async getSchemas(): Promise<Schema[]> {
    return schemas;
  }
}
