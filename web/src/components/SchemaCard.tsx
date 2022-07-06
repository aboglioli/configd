import { FC, ReactNode } from 'react';

import { Schema } from 'domain/schema';

import './SchemaCard.css';

export interface SchemaCardProps {
  schema: Schema;
  children: ReactNode;
}

export const SchemaCard: FC<SchemaCardProps> = ({ schema, children }) => {
  return (
    <div className="schema">
      <div className="schema__subtitle">
        # <a href="#">{schema.id}</a>
      </div>
      <div className="schema__title">
        <img className="schema__icon" src="/schema.png" />
        <h2>{schema.name}</h2>
      </div>
      <div className="schema__content">{children}</div>
      <div className="schema__footer">
        <div className="schema__footer__buttons">
          <button>Edit</button>
          <button className="primary">Create config</button>
        </div>
      </div>
    </div>
  );
};
