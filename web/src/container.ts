import { InMemSchemaService } from 'infrastructure/inmem-schema-service';

export class Container {
  private static container: Container;

  private constructor(public schemaService: InMemSchemaService) {}

  static get(): Container {
    if (!this.container) {
      this.container = new Container(new InMemSchemaService());
    }

    return this.container;
  }
}
