import { FC } from 'react';

import { Schema } from 'domain/schema';

import './SchemaCard.css';

export interface SchemaCardProps {
  schema: Schema;
}

export const SchemaCard: FC<SchemaCardProps> = ({ schema }) => {
  return (
    <div className="schema">
      <div className="schema__title">
        <h2>{schema.name}</h2>
      </div>
      <div className="schema__subtitle">{schema.id}</div>
      <div className="schema__content">
        <ul>
          {schema.configs.map((config) => (
            <li key={config.id}>
              {config.name} ({config.id}): {config.valid && 'Valid'} | Checksum:{' '}
              {config.checksum}
            </li>
          ))}
        </ul>
        <small>({schema.configs.length} configurations.)</small>
      </div>
      <div className="schema__footer">
        <small>Created: {schema.created_at.toISOString()}</small>
        <small>· Updated: {schema.updated_at.toISOString()}</small>
        <small>· Version: {schema.version}</small>
      </div>
    </div>
  );
};
