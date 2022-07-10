import { FC, useEffect } from 'react';

import { useSchemas } from 'hooks/schema';
import { VerticalWrapper, Wrapper } from 'styles/Wrapper';
import { Size } from 'styles/Box';
import { Title } from 'styles/Title';
import { Input } from 'styles/Form';
import { Button } from 'styles/Button';
import {
  List,
  ListItem,
  ListItemImage,
  ListItemContent,
  ListItemButtons,
} from 'styles/List';

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
    <VerticalWrapper gap={Size.Medium}>
      <Wrapper bordered padding={Size.Medium}>
        <Title>Schemas</Title>
      </Wrapper>

      <Wrapper bordered padding={Size.Medium} gap={Size.Small}>
        <Input placeholder="Search schemas..." />
        <Button primary>New schema</Button>
      </Wrapper>

      <List bordered padding={Size.Medium} gap={Size.Small}>
        {schemas.map((schema) => (
          <ListItem key={schema.id} bordered padding={Size.Small} highlightOnHover>
            <ListItemImage src="schema.png" />
            <ListItemContent>
              <h3>{schema.name}</h3>
              <small>{schema.configs.length} configurations.</small>
            </ListItemContent>
            <ListItemButtons>
              <Button onClick={viewSchema}>View</Button>
              <Button primary onClick={createConfig}>
                New config
              </Button>
            </ListItemButtons>
          </ListItem>
        ))}
      </List>
    </VerticalWrapper>
  );
};

export default Schemas;
