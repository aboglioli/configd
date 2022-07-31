import { FC, useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { AiOutlineSave } from 'react-icons/ai';
import { CgEditBlackPoint } from 'react-icons/cg';
import dayjs from 'dayjs';
import { AxiosError } from 'axios';

import { Container } from 'container';
import { Config } from 'domain/config';
import { Button } from 'styles/Form';
import { ListItem, ListItemContent } from 'styles/List';
import { Message } from 'styles/Message';
import { Size, Alignment } from 'styles/common';
import { Title, Subtitle, SmallTitle, SmallestTitle } from 'styles/Title';
import { Wrapper } from 'styles/Wrapper';
import { ConfigData } from 'components/ConfigData';

export interface ConfigProps {
  setTitle: (title: string) => void;
}

const ConfigPage: FC<ConfigProps> = ({ setTitle }) => {
  useEffect(() => {
    setTitle('Config');
  }, []);

  const { schemaService } = Container.get();

  const { schemaId, configId } = useParams();
  const [config, setConfig] = useState<Config>();
  const [dataExpanded, setDataExpanded] = useState(false);
  const [configValid, setConfigValid] = useState(true);
  const [error, setError] = useState<string>();

  const loadConfig = async () => {
    if (schemaId && configId) {
      const res = await schemaService.getConfig(schemaId, configId);
      setConfig(res);
    }
  };

  useEffect(() => {
    loadConfig();
  }, [schemaId, configId]);

  if (!config) {
    return <b>Loading...</b>;
  }

  const handleDataChange = (valid: boolean, data?: unknown) => {
    setConfigValid(valid);

    if (valid && data) {
      setConfig((config) => (config ? { ...config, data } : config));
      setError('');
    }
  };

  const handleConfigSave = async () => {
    try {
      await schemaService.updateConfig(config.schema_id, config.id, {
        data: config.data,
      });
      setError('');

      loadConfig();
    } catch (err) {
      console.error(err);
      if (err instanceof AxiosError) {
        setError(err.response?.data?.message);
        return;
      }

      setError('Invalid schema');
    }
  };

  const toggleSchemaVisibility = () => {
    setDataExpanded((dataExpanded) => !dataExpanded);
  };

  return (
    <Wrapper $vertical $gap={Size.Medium}>
      <Wrapper
        $bordered
        $padding={Size.Medium}
        $verticalAlignment={Alignment.Center}
        $gap={Size.Medium}
      >
        <img src="/gears.png" style={{ height: '32px', width: 'auto' }} />
        <Wrapper $vertical>
          <Title>{config.name}</Title>
          <SmallTitle>{config.id}</SmallTitle>
        </Wrapper>
      </Wrapper>

      <Wrapper $bordered $padding={Size.Medium} $vertical $gap={Size.Medium}>
        <Wrapper>
          <Subtitle style={{ flex: 1 }}>Data</Subtitle>
          {dataExpanded ? (
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
        {dataExpanded && (
          <>
            <ConfigData data={config.data} onChange={handleDataChange} />
            <Wrapper $alignment={Alignment.End}>
              <Button $primary disabled={!configValid} onClick={handleConfigSave}>
                <AiOutlineSave />
                Save
              </Button>
            </Wrapper>
          </>
        )}
      </Wrapper>

      <Wrapper $bordered $padding={Size.Medium} $vertical $gap={Size.Small}>
        <Subtitle>Accesses</Subtitle>
        {config.accesses.map((access) => (
          <ListItem key={config.id} $bordered $padding={Size.Small}>
            <CgEditBlackPoint />
            <ListItemContent>
              <Wrapper $verticalAlignment={Alignment.Center} $gap={Size.Small}>
                <SmallTitle>{access.source}</SmallTitle>
                <SmallestTitle>{access.instance}</SmallestTitle>
              </Wrapper>
            </ListItemContent>
          </ListItem>
        ))}
      </Wrapper>

      <Wrapper $alignment={Alignment.End}>
        <p style={{ fontSize: '0.8rem' }}>
          <b>Version</b>: {config.version} · <b>Created</b>:{' '}
          {dayjs(config.created_at).format('DD/MM/YYYY HH:mm')} · <b>Updated</b>:{' '}
          {dayjs(config.updated_at).format('DD/MM/YYYY HH:mm')}
        </p>
      </Wrapper>
    </Wrapper>
  );
};

export default ConfigPage;
