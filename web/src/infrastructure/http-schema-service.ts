import axios from 'axios';

import { Schema } from 'domain/schema';
import {
  SchemaService,
  UpdateSchemaCommand,
  UpdateSchemaResponse,
} from 'domain/schema-service';
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

  async updateSchema(
    schemaId: string,
    cmd: UpdateSchemaCommand,
  ): Promise<UpdateSchemaResponse> {
    const res = await axios.put(`${this.baseUrl}/schemas/${schemaId}`, cmd);
    return res.data;
  }
}
