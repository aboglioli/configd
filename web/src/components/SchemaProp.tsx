import { FC } from 'react';

import { RootProp } from 'domain/schema';
import { Wrapper } from 'styles/Wrapper';
import { Size } from 'styles/common';

export interface SchemaPropProps {
  name?: string;
  prop: RootProp;
}

export const SchemaProp: FC<SchemaPropProps> = ({ name, prop }) => {
  if (Array.isArray(prop)) {
    return (
      <>
        {prop.map((prop, i) => (
          <SchemaProp key={i} prop={prop} />
        ))}
      </>
    );
  }

  if (name === '$schema') {
    return (
      <Wrapper $bordered $gap={Size.Small} $padding={Size.Small}>
        <b>{name}</b>
        <small>{JSON.stringify(prop)}</small>
      </Wrapper>
    );
  }

  // if (name && typeof prop === 'object') {
  //   return (
  //     <>
  //       {Object.entries(prop).map(([name, prop]) => (
  //         <SchemaProp key={name} name={name} prop={prop} />
  //       ))}
  //     </>
  //   );
  // }

  return (
    <>
      {Object.entries(prop).map(([name, prop]) => (
        <Wrapper key={name} $bordered $gap={Size.Small} $vertical $padding={Size.Small}>
          <b>{name}</b>
          <SchemaProp key={name} name={name} prop={prop} />
        </Wrapper>
      ))}
    </>
  );
};
