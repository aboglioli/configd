import axios from 'axios';

import { Schema } from 'domain/schema';
import { Config } from 'domain/config';
import {
  SchemaService,
  UpdateSchemaCommand,
  UpdateSchemaResponse,
  UpdateConfigCommand,
  UpdateConfigResponse,
} from 'domain/schema-service';
import { Page } from 'domain/page';

export class HttpSchemaService implements SchemaService {
  constructor(private baseUrl: string, private defaultHeaders: Record<string, string>) {}

  // Schema
  async getSchemas(): Promise<Page<Schema>> {
    const res = await axios.get(`${this.baseUrl}/schemas`, {
      headers: this.defaultHeaders,
    });
    return res.data;
  }

  async getSchema(schemaId: string): Promise<Schema> {
    const res = await axios.get(`${this.baseUrl}/schemas/${schemaId}`, {
      headers: this.defaultHeaders,
    });
    return res.data;
  }

  async updateSchema(
    schemaId: string,
    cmd: UpdateSchemaCommand,
  ): Promise<UpdateSchemaResponse> {
    const res = await axios.put(`${this.baseUrl}/schemas/${schemaId}`, cmd, {
      headers: this.defaultHeaders,
    });
    return res.data;
  }

  // Config
  async getConfig(
    schemaId: string,
    configId: string,
    password?: string,
  ): Promise<Config> {
    const res = await axios.get(
      `${this.baseUrl}/schemas/${schemaId}/configs/${configId}`,
      {
        headers: {
          ...this.defaultHeaders,
          ...(password && { 'X-Configd-Password': password }),
        },
      },
    );
    return res.data;
  }

  async updateConfig(
    schemaId: string,
    configId: string,
    cmd: UpdateConfigCommand,
    password?: string,
  ): Promise<UpdateConfigResponse> {
    const res = await axios.put(
      `${this.baseUrl}/schemas/${schemaId}/configs/${configId}`,
      cmd,
      {
        headers: {
          ...this.defaultHeaders,
          ...(password && { 'X-Configd-Password': password }),
        },
      },
    );
    return res.data;
  }
}
