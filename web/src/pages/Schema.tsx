import { FC, useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { AiOutlineSave } from 'react-icons/ai';
import { BiEdit } from 'react-icons/bi';
import dayjs from 'dayjs';

import { Container } from 'container';
import { RootProp, Schema } from 'domain/schema';
import { Button, ButtonLink } from 'styles/Form';
import { ListItem, ListItemImage, ListItemContent, ListItemButtons } from 'styles/List';
import { Message } from 'styles/Message';
import { Size, Alignment } from 'styles/common';
import { Title, Subtitle, SmallTitle, SmallestTitle } from 'styles/Title';
import { Wrapper } from 'styles/Wrapper';
import { SchemaProp } from 'components/SchemaProp';

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
  const [schemaExpanded, setSchemaExpanded] = useState(false);
  const [schemaValid, setSchemaValid] = useState(true);
  const [error, setError] = useState<string>();

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
      setError('');
    }
  };

  const handleSchemaSave = async () => {
    try {
      const res = await schemaService.updateSchema(schema.id, { schema: schema.schema });
      console.log(res);
      setError('');
    } catch (err) {
      setError('Invalid schema');
    }
  };

  const toggleSchemaVisibility = () => {
    setSchemaExpanded((schemaExpanded) => !schemaExpanded);
  };

  return (
    <Wrapper $vertical $gap={Size.Medium}>
      <Wrapper $alignment={Alignment.End}>
        <p style={{ fontSize: '0.8rem' }}>
          <b>Version</b>: {schema.version} · <b>Created</b>:{' '}
          {dayjs(schema.created_at).format('DD/MM/YYYY HH:mm')} · <b>Updated</b>:{' '}
          {dayjs(schema.updated_at).format('DD/MM/YYYY HH:mm')}
        </p>
      </Wrapper>

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
        <Wrapper>
          <Subtitle style={{ flex: 1 }}>Schema</Subtitle>
          {schemaExpanded ? (
            <Button $size={Size.Small} onClick={toggleSchemaVisibility}>
              Hide
            </Button>
          ) : (
            <Button $size={Size.Small} onClick={toggleSchemaVisibility}>
              Expand
            </Button>
          )}
        </Wrapper>
        {error && <Message $error>{error}</Message>}
        {schemaExpanded && (
          <>
            <SchemaProp prop={schema.schema} onChange={handlePropChange} />
            <Wrapper $alignment={Alignment.End}>
              <Button $primary disabled={!schemaValid} onClick={handleSchemaSave}>
                <AiOutlineSave />
                Save
              </Button>
            </Wrapper>
          </>
        )}
      </Wrapper>

      <Wrapper $bordered $padding={Size.Medium} $vertical $gap={Size.Small}>
        <Subtitle>Configs</Subtitle>
        {schema.configs.map((config) => (
          <ListItem key={config.id} $bordered $padding={Size.Small}>
            <ListItemImage src="/gears.png" />
            <ListItemContent>
              <Wrapper $verticalAlignment={Alignment.Center} $gap={Size.Small}>
                <SmallTitle>{config.name}</SmallTitle>
                <SmallestTitle>{config.id}</SmallestTitle>
              </Wrapper>
            </ListItemContent>
            <ListItemButtons>
              <ButtonLink
                to={`/schemas/${schema.id}/configs/${config.id}`}
                $size={Size.Small}
              >
                <BiEdit />
                View
              </ButtonLink>
            </ListItemButtons>
          </ListItem>
        ))}
      </Wrapper>
    </Wrapper>
  );
};

export default SchemaPage;
