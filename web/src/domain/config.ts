export interface Config {
  id: string;
  name: string;
  data?: unknown;
  valid: boolean;
  checksum: string;
  created_at: Date;
  updated_at: Date;
  version: number;
}
