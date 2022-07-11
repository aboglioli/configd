import { FC, useEffect } from 'react';
// import { FaBeer } from 'react-icons/fa';
import { BsDiagram3, BsSearch } from 'react-icons/bs';
import { GrDocumentConfig } from 'react-icons/gr';
import { BiEdit } from 'react-icons/bi';

import { useSchemas } from 'hooks/schemas';
import { Wrapper } from 'styles/Wrapper';
import { Size, Alignment } from 'styles/common';
import { Title } from 'styles/Title';
import { Input, Button } from 'styles/Form';
import { ListItem, ListItemImage, ListItemContent, ListItemButtons } from 'styles/List';

export interface SchemasProps {
  setTitle: (title: string) => void;
}

const Schemas: FC<SchemasProps> = ({ setTitle }) => {
  useEffect(() => {
    setTitle('Home');
  }, []);

  const { loading, data: schemasPage } = useSchemas();
  console.log(loading, schemasPage);

  if (loading || !schemasPage) {
    return <b>Loading...</b>;
  }

  const schemas = schemasPage.data;

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

      <Wrapper
        bordered
        padding={Size.Medium}
        gap={Size.Small}
        verticalAlignment={Alignment.Center}
      >
        <BsSearch />
        <Input placeholder="Search schemas..." />
        <Button primary>
          <BsDiagram3 />
          Create schema
        </Button>
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
              <Button onClick={viewSchema}>
                <BiEdit />
                View
              </Button>
              <Button primary onClick={createConfig}>
                <GrDocumentConfig />
                Create config
              </Button>
            </ListItemButtons>
          </ListItem>
        ))}
      </Wrapper>
    </Wrapper>
  );
};

export default Schemas;
