import axios from 'axios';

import { SchemaService, Schema } from 'domain/schema';
import { Page } from 'domain/page';

export class HttpSchemaService implements SchemaService {
  constructor(private baseUrl: string) {}

  async getSchemas(): Promise<Page<Schema>> {
    const res = await axios.get(`${this.baseUrl}/schemas`);
    return res.data;
  }

  async getSchema(schemaId: string): Promise<Schema> {
    const res = await axios.get(`${this.baseUrl}/schemas/${schemaId}`);
    return res.data;
  }
}
