import { FC, useState, useEffect } from 'react';
import AceEditor from 'react-ace';

import 'ace-builds/src-noconflict/mode-json';
import 'ace-builds/src-noconflict/theme-github';
import 'ace-builds/src-noconflict/ext-language_tools';

import { RootProp } from 'domain/schema';

export interface SchemaPropProps {
  prop: RootProp;
  onChange: (valid: boolean, prop?: RootProp) => void;
}

export const SchemaProp: FC<SchemaPropProps> = ({ prop, onChange }) => {
  const [json, setJson] = useState(JSON.stringify(prop, null, 2));
  useEffect(() => {
    setJson(JSON.stringify(prop, null, 2));
  }, []);

  const onAceChange = (json: string) => {
    setJson(json);

    try {
      const prop = JSON.parse(json);
      onChange(true, prop);
    } catch (err) {
      onChange(false);
    }
  };

  return (
    <AceEditor
      mode="json"
      theme="github"
      name="schema-prop"
      editorProps={{ $blockScrolling: true }}
      width="100%"
      showGutter={true}
      showPrintMargin={true}
      value={json}
      onChange={onAceChange}
      setOptions={{
        enableBasicAutocompletion: true,
        enableLiveAutocompletion: true,
        enableSnippets: true,
        tabSize: 2,
      }}
    />
  );
};
