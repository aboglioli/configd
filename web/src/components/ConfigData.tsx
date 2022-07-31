import { FC, useState, useEffect } from 'react';
import AceEditor from 'react-ace';

import 'ace-builds/src-noconflict/mode-json';
import 'ace-builds/src-noconflict/theme-github';
import 'ace-builds/src-noconflict/ext-language_tools';

export interface ConfigDataProps {
  data: unknown;
  onChange: (valid: boolean, data?: unknown) => void;
}

export const ConfigData: FC<ConfigDataProps> = ({ data, onChange }) => {
  const [json, setJson] = useState(JSON.stringify(data, null, 2));
  useEffect(() => {
    setJson(JSON.stringify(data, null, 2));
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
