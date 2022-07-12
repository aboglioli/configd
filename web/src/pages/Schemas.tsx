import { FC, useEffect } from 'react';
import { BsDiagram3 } from 'react-icons/bs';
import { GrDocumentConfig } from 'react-icons/gr';
import { BiEdit } from 'react-icons/bi';

import { useSchemas } from 'hooks/schemas';
import { Wrapper } from 'styles/Wrapper';
import { Size, Alignment } from 'styles/common';
import { Title, SmallTitle } from 'styles/Title';
import { Input, ButtonLink } from 'styles/Form';
import { ListItem, ListItemImage, ListItemContent, ListItemButtons } from 'styles/List';

export interface SchemasProps {
  setTitle: (title: string) => void;
}

const SchemasPage: FC<SchemasProps> = ({ setTitle }) => {
  useEffect(() => {
    setTitle('Home');
  }, []);

  const { loading, data: schemasPage } = useSchemas();

  if (loading || !schemasPage) {
    return <b>Loading...</b>;
  }

  const schemas = schemasPage.data;

  return (
    <Wrapper $vertical $gap={Size.Medium}>
      <Wrapper $bordered $padding={Size.Medium}>
        <Title>Schemas</Title>
      </Wrapper>

      <Wrapper
        $bordered
        $padding={Size.Medium}
        $gap={Size.Small}
        $verticalAlignment={Alignment.Center}
      >
        <Input placeholder="Search schemas..." $size={Size.Medium} />
        <ButtonLink to="/schemas" $primary $size={Size.Medium}>
          <BsDiagram3 />
          Create schema
        </ButtonLink>
      </Wrapper>

      <Wrapper $bordered $padding={Size.Medium} $gap={Size.Small}>
        {schemas.map((schema) => (
          <ListItem key={schema.id} $bordered $padding={Size.Small}>
            <ListItemImage src="schema.png" />
            <ListItemContent>
              <SmallTitle>{schema.name}</SmallTitle>
              <small>
                {schema.configs.length} configurations:{' '}
                <b>{schema.configs.map((config) => config.id).join(', ')}</b>.
              </small>
            </ListItemContent>
            <ListItemButtons>
              <ButtonLink to={`/schemas/${schema.id}`} $size={Size.Small}>
                <BiEdit />
                View
              </ButtonLink>
              <ButtonLink to={`/schemas/${schema.id}/config`} $size={Size.Small} $primary>
                <GrDocumentConfig />
                Create config
              </ButtonLink>
            </ListItemButtons>
          </ListItem>
        ))}
      </Wrapper>
    </Wrapper>
  );
};

export default SchemasPage;
