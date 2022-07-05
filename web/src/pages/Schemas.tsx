import { SchemaCard } from 'components/SchemaCard';
import { Schema } from 'domain/schema';

import './Schemas.css';

const schemas: Schema[] = [
  {
    id: 'custom-schema',
    name: 'Custom Schema',
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
    ],
    created_at: new Date(),
    updated_at: new Date(),
    version: 1,
  },
];

const Schemas = () => {
  return (
    <div className="schemas">
      <header className="schemas__header">
        <h1>Schemas</h1>
      </header>
      <div className="schemas__content">
        {schemas.map((schema) => (
          <SchemaCard key={schema.id} schema={schema} />
        ))}
      </div>
    </div>
  );
};

export default Schemas;
