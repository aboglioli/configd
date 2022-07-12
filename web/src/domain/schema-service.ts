import { Page } from 'domain/page';
import { Schema, RootProp } from 'domain/schema';

export interface UpdateSchemaCommand {
  schema: RootProp;
}

export interface UpdateSchemaResponse {
  id: string;
}

export interface SchemaService {
  getSchemas(): Promise<Page<Schema>>;
  getSchema(schemaId: string): Promise<Schema>;
  updateSchema(schemaId: string, cmd: UpdateSchemaCommand): Promise<UpdateSchemaResponse>;
}
