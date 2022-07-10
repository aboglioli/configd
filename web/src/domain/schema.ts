import { Prop } from 'domain/prop';

export interface SchemaService {
  getSchemas(): Promise<Schema[]>;
}

export type RootProp = { $schema: Prop } | RootProp[] | { [key: string]: RootProp };

export interface SchemaConfig {
  id: string;
  name: string;
  data?: unknown;
  valid: boolean;
  checksum: string;
  created_at: Date;
  updated_at: Date;
  version: number;
}

export interface Schema {
  id: string;
  name: string;
  schema: RootProp;
  configs: SchemaConfig[];
  created_at: Date;
  updated_at: Date;
  version: number;
}
