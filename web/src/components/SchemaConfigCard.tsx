import { FC } from 'react';

import { SchemaConfig } from 'domain/schema';

import './SchemaConfigCard.css';

export interface Props {
  config: SchemaConfig;
}

export const SchemaConfigCard: FC<Props> = ({ config }) => {
  return (
    <div className="schema-config">
      <div className="schema-config__subtitle">
        # <a href="#">{config.id}</a>
      </div>
      <div className="schema-config__title">
        <img className="schema-config__icon" src="/gears.png" />
        <h3>{config.name}</h3>
      </div>
      <div className="schema-config__content">Content</div>
      <div className="schema-config__footer">
        <small>Footer</small>
      </div>
    </div>
  );
};
