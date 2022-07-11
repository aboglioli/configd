import styled, { css } from 'styled-components';

import { Size } from 'styles/common';

const sizes = {
  [Size.Small]: '0.5rem',
  [Size.Medium]: '1rem',
  [Size.Large]: '1.5rem',
};

export interface BoxProps {
  $bordered?: boolean;
  $gap?: Size;
  $padding?: Size;
}

export const Box = styled.div<BoxProps>`
  ${(props) =>
    props.$padding
      ? css`
          padding: ${sizes[props.$padding]};
        `
      : ''}
  ${(props) =>
    props.$gap
      ? css`
          gap: ${sizes[props.$gap]};
        `
      : ''}
  ${(props) =>
    props.$bordered
      ? css`
          border: 1px solid var(--lighter-color);
        `
      : ''}
`;
