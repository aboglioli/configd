import { Page } from 'domain/page';
import { Schema, RootProp } from 'domain/schema';
import { Config } from 'domain/config';

export interface UpdateSchemaCommand {
  schema: RootProp;
}

export interface UpdateSchemaResponse {
  id: string;
}

export interface UpdateConfigCommand {
  data: unknown;
}

export interface UpdateConfigResponse {
  schema_id: string;
  config_id: string;
}

export interface SchemaService {
  getSchemas(): Promise<Page<Schema>>;
  getSchema(schemaId: string): Promise<Schema>;
  updateSchema(schemaId: string, cmd: UpdateSchemaCommand): Promise<UpdateSchemaResponse>;

  getConfig(schemaId: string, configId: string, password?: string): Promise<Config>;
  updateConfig(
    schemaId: string,
    configId: string,
    cmd: UpdateConfigCommand,
    password?: string,
  ): Promise<UpdateConfigResponse>;
}
