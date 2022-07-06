import { FC, useEffect } from 'react';

import { SchemaCard } from 'components/SchemaCard';
import { SchemaConfigCard } from 'components/SchemaConfigCard';
import { Schema } from 'domain/schema';
import { PropKind } from 'domain/prop';

import './Schemas.css';

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

export interface SchemasProps {
  setTitle: (title: string) => void;
}

const Schemas: FC<SchemasProps> = ({ setTitle }) => {
  useEffect(() => {
    setTitle('Home');
  }, []);

  return (
    <div className="schemas">
      <div className="schemas__content">
        {schemas.map((schema) => (
          <SchemaCard key={schema.id} schema={schema}>
            {schema.configs.map((config) => (
              <SchemaConfigCard key={config.id} config={config} />
            ))}
          </SchemaCard>
        ))}
      </div>
    </div>
  );
};

export default Schemas;
