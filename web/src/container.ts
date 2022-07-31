import { HttpSchemaService } from 'infrastructure/http-schema-service';

export class Container {
  private static container: Container;

  private constructor(public schemaService: HttpSchemaService) {}

  static get(): Container {
    if (!this.container) {
      this.container = new Container(
        new HttpSchemaService('http://localhost:8080', {
          'X-Configd-Source': 'Configd Web',
          'X-Configd-Instance': 'v0.1.0',
        }),
      );
    }

    return this.container;
  }
}
