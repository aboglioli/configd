export interface Interval {
  min?: number;
  max?: number;
}

export enum PropKind {
  Bool = 'bool',
  Int = 'int',
  Float = 'float',
  String = 'string',
}

export interface Prop {
  kind: PropKind;
  required: boolean;
  default_value?: unknown;
  allowed_values?: unknown[];
  interval?: Interval;
  regex?: string;
  split?: boolean;
}
