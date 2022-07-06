export type PrimitiveValue = boolean | number | string;

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

export interface BoolProp {
  kind: PropKind.Bool;
  required: boolean;
  default_value?: boolean;
}

export interface IntProp {
  kind: PropKind.Int;
  required: boolean;
  allowed_values?: number[];
  interval?: Interval;
}

export interface FloatProp {
  kind: PropKind.Float;
  required: boolean;
  allowed_values?: number[];
  interval?: Interval;
}

export interface StringProp {
  kind: PropKind.String;
  required: boolean;
  default_value?: string;
  allowed_values?: string[];
  regex?: string;
}

export type Prop = BoolProp | IntProp | FloatProp | StringProp;
