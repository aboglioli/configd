import { FC, useEffect } from 'react';

import { useSchemas } from 'hooks/schema';
import { Wrapper } from 'styles/Wrapper';
import { Size } from 'styles/common';
import { Title } from 'styles/Title';
import { Input, Button } from 'styles/Form';
import { ListItem, ListItemImage, ListItemContent, ListItemButtons } from 'styles/List';

export interface SchemasProps {
  setTitle: (title: string) => void;
}

const Schemas: FC<SchemasProps> = ({ setTitle }) => {
  const schemas = useSchemas();

  useEffect(() => {
    setTitle('Home');
  }, []);

  const viewSchema = () => {
    console.log('View schema');
  };

  const createConfig = () => {
    console.log('Create config');
  };

  return (
    <Wrapper vertical gap={Size.Medium}>
      <Wrapper bordered padding={Size.Medium}>
        <Title>Schemas</Title>
      </Wrapper>

      <Wrapper bordered padding={Size.Medium} gap={Size.Small}>
        <Input placeholder="Search schemas..." />
        <Button primary>New schema</Button>
      </Wrapper>

      <Wrapper bordered padding={Size.Medium} gap={Size.Small}>
        {schemas.map((schema) => (
          <ListItem key={schema.id} bordered padding={Size.Small}>
            <ListItemImage src="schema.png" />
            <ListItemContent>
              <h3>{schema.name}</h3>
              <small>
                {schema.configs.length} configurations:{' '}
                <b>{schema.configs.map((config) => config.id).join(', ')}</b>.
              </small>
            </ListItemContent>
            <ListItemButtons>
              <Button onClick={viewSchema}>View</Button>
              <Button primary onClick={createConfig}>
                New config
              </Button>
            </ListItemButtons>
          </ListItem>
        ))}
      </Wrapper>
    </Wrapper>
  );
};

export default Schemas;
