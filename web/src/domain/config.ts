export interface Access {
  source: string;
  instance: string;
  timestamp: Date;
  previous?: Date;
}

export interface Config {
  schema_id: string;
  id: string;
  name: string;
  data: unknown;
  valid: boolean;
  checksum: string;
  requires_password: boolean;
  accesses: Access[];
  created_at: Date;
  updated_at: Date;
  version: number;
}
