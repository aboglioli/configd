import { Container } from 'container';
import { Schema } from 'domain/schema';
import { Page } from 'domain/page';
import { useRequest, Response } from 'hooks/request';

export const useSchemas = (): Response<Page<Schema>> => {
  const { schemaService } = Container.get();

  return useRequest(() => schemaService.getSchemas());
};
