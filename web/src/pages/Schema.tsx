import { FC, useEffect } from 'react';
import { useParams } from 'react-router-dom';

import { useSchema } from 'hooks/schemas';
import { Wrapper } from 'styles/Wrapper';
import { Title, Subtitle, SmallTitle } from 'styles/Title';
import { Size } from 'styles/common';

export interface SchemaProps {
  setTitle: (title: string) => void;
}

const Schema: FC<SchemaProps> = ({ setTitle }) => {
  useEffect(() => {
    setTitle('Schema');
  }, []);

  const { schemaId } = useParams();

  if (!schemaId) {
    return <b>Missing schemaId</b>;
  }

  const { loading, data: schema } = useSchema(schemaId);

  if (loading || !schema) {
    return <b>Loading...</b>;
  }

  return (
    <Wrapper $vertical $gap={Size.Medium}>
      <Wrapper $bordered $padding={Size.Medium} $vertical>
        <Title>{schema.name}</Title>
        <SmallTitle>{schema.id}</SmallTitle>
      </Wrapper>
      <Wrapper $bordered $padding={Size.Medium} $vertical>
        <Subtitle>Schema</Subtitle>
        <b>{schema.configs.length}</b>
      </Wrapper>
    </Wrapper>
  );
};

export default Schema;
