export interface Page<T> {
  offset: number;
  limit: number;
  total: number;
  data: T[];
}
