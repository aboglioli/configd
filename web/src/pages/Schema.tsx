import { FC, useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { AiOutlineSave } from 'react-icons/ai';

import { Wrapper } from 'styles/Wrapper';
import { Title, Subtitle, SmallTitle } from 'styles/Title';
import { Size, Alignment } from 'styles/common';
import { SchemaProp } from 'components/SchemaProp';
import { Button } from 'styles/Form';
import { RootProp, Schema } from 'domain/schema';
import { Container } from 'container';

export interface SchemaProps {
  setTitle: (title: string) => void;
}

const SchemaPage: FC<SchemaProps> = ({ setTitle }) => {
  useEffect(() => {
    setTitle('Schema');
  }, []);

  const { schemaService } = Container.get();

  const { schemaId } = useParams();
  const [schema, setSchema] = useState<Schema>();
  const [schemaValid, setSchemaValid] = useState(true);

  useEffect(() => {
    if (schemaId) {
      (async () => {
        const res = await schemaService.getSchema(schemaId);
        setSchema(res);
      })();
    }
  }, [schemaId]);

  if (!schema) {
    return <b>Loading...</b>;
  }

  const handlePropChange = (_json: string, valid: boolean, prop?: RootProp) => {
    setSchemaValid(valid);

    if (valid && prop) {
      setSchema((schema) => (schema ? { ...schema, schema: prop } : schema));
    }
  };

  const handleSchemaSave = () => {
    console.log('Schema save');
  };

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
      <Wrapper $bordered $padding={Size.Medium} $vertical $gap={Size.Medium}>
        <Subtitle>Schema</Subtitle>
        <SchemaProp prop={schema.schema} onChange={handlePropChange} />
        <Wrapper $alignment={Alignment.End}>
          <Button $primary disabled={!schemaValid} onClick={handleSchemaSave}>
            <AiOutlineSave />
            Save
          </Button>
        </Wrapper>
      </Wrapper>
    </Wrapper>
  );
};

export default SchemaPage;
