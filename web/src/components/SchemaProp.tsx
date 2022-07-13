import { FC, ChangeEvent, useState } from 'react';

import { RootProp } from 'domain/schema';
import { TextArea, Button } from 'styles/Form';
import { Size } from 'styles/common';
import { Wrapper } from 'styles/Wrapper';

export interface SchemaPropProps {
  prop: RootProp;
  onChange: (rawJson: string, valid: boolean, prop?: RootProp) => void;
}

export const SchemaProp: FC<SchemaPropProps> = ({ prop, onChange }) => {
  const [valid, setValid] = useState(true);
  const [jsonProp, setJsonProp] = useState(JSON.stringify(prop, null, 4));

  const handleChange = (event: ChangeEvent<HTMLTextAreaElement>) => {
    setJsonProp(event.target.value);

    try {
      const newProp = JSON.parse(event.target.value);

      onChange(event.target.value, true, newProp);
      setValid(true);
    } catch (err) {
      onChange(event.target.value, false);
      setValid(false);
    }
  };

  const format = () => {
    setJsonProp(JSON.stringify(prop, null, 4));
    setValid(true);
  };

  return (
    <Wrapper $vertical>
      <TextArea
        $size={Size.Small}
        style={{
          backgroundColor: valid ? 'rgba(0, 255, 0, 0.05)' : 'rgba(255, 0, 0, 0.05)',
        }}
        value={jsonProp}
        onChange={handleChange}
      />
      <Button $size={Size.Small} onClick={format}>
        Format
      </Button>
    </Wrapper>
  );
};
