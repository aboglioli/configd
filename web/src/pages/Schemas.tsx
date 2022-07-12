import { FC, useEffect, useState } from 'react';
import { BsDiagram3 } from 'react-icons/bs';
import { GrDocumentConfig } from 'react-icons/gr';
import { BiEdit } from 'react-icons/bi';

import { Container } from 'container';
import { Page } from 'domain/page';
import { Schema } from 'domain/schema';
import { Input, ButtonLink } from 'styles/Form';
import { ListItem, ListItemImage, ListItemContent, ListItemButtons } from 'styles/List';
import { Size, Alignment } from 'styles/common';
import { Title, SmallTitle, SmallestTitle } from 'styles/Title';
import { Wrapper } from 'styles/Wrapper';

export interface SchemasProps {
  setTitle: (title: string) => void;
}

const SchemasPage: FC<SchemasProps> = ({ setTitle }) => {
  useEffect(() => {
    setTitle('Home');
  }, []);

  const { schemaService } = Container.get();
  const [schemasPage, setSchemasPage] = useState<Page<Schema>>();

  useEffect(() => {
    (async () => {
      const res = await schemaService.getSchemas();
      setSchemasPage(res);
    })();
  }, []);

  if (!schemasPage) {
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
            <ListItemImage src="/schema.png" />
            <ListItemContent>
              <Wrapper $verticalAlignment={Alignment.Center} $gap={Size.Small}>
                <SmallTitle>{schema.name}</SmallTitle>
                <SmallestTitle>{schema.id}</SmallestTitle>
              </Wrapper>
              <small>
                {schema.configs.length} configurations:{' '}
                {schema.configs.map((config) => config.id).join(', ')}.
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
