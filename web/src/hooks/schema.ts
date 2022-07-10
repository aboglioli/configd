import { useState, useEffect } from 'react';

import { Container } from 'container';
import { Schema } from 'domain/schema';

export const useSchemas = (): Schema[] => {
  const [schemas, setSchemas] = useState<Schema[]>([]);
  const { schemaService } = Container.get();

  useEffect(() => {
    (async () => {
      const schemas = await schemaService.getSchemas();
      setSchemas(schemas);
    })();
  }, []);

  return schemas;
};
