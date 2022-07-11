import { useState, useEffect } from 'react';

export interface Response<T> {
  loading: boolean;
  error?: Error;
  data?: T;
}

export type Request<T> = () => Promise<T>;

export const useRequest = <T>(request: Request<T>): Response<T> => {
  const [response, setResponse] = useState({
    loading: true,
  });

  useEffect(() => {
    (async () => {
      try {
        setResponse((response) => ({ ...response, loading: true }));

        const res = await request();

        setResponse((response) => ({ ...response, loading: false, data: res }));
      } catch (err) {
        setResponse((response) => ({ ...response, loading: false, error: err }));
      }
    })();
  }, []);

  return response;
};
