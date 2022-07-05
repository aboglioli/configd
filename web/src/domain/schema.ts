import { Config } from 'domain/config';

export interface Schema {
  id: string;
  name: string;
  configs: Config[];
  created_at: Date;
  updated_at: Date;
  version: number;
}
