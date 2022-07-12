import { FC, useEffect } from 'react';
import { useParams } from 'react-router-dom';

import { useSchema } from 'hooks/schemas';
import { Wrapper } from 'styles/Wrapper';
import { Title, Subtitle, SmallTitle } from 'styles/Title';
import { Size, Alignment } from 'styles/common';
import { SchemaProp } from 'components/SchemaProp';

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
      <Wrapper
        $bordered
        $padding={Size.Medium}
        $verticalAlignment={Alignment.Center}
        $gap={Size.Medium}
      >
        <img src="/schema.png" style={{ height: '32px', width: 'auto' }} />
        <Wrapper $vertical>
          <Title>{schema.name}</Title>
          <SmallTitle>{schema.id}</SmallTitle>
        </Wrapper>
      </Wrapper>
      <Wrapper $bordered $padding={Size.Medium} $vertical>
        <Subtitle>Schema</Subtitle>
        <SchemaProp prop={schema.schema} />
      </Wrapper>
    </Wrapper>
  );
};

export default Schema;
