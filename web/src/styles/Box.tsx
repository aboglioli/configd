import styled, { css } from 'styled-components';

import { Size, sizes } from 'styles/common';

export interface BoxProps {
  bordered?: boolean;
  gap?: Size;
  padding?: Size;
}

export const Box = styled.div<BoxProps>`
  ${(props) =>
    props.padding
      ? css`
          padding: ${sizes[props.padding]};
        `
      : ''}
  ${(props) =>
    props.gap
      ? css`
          gap: ${sizes[props.gap]};
        `
      : ''}
  ${(props) =>
    props.bordered
      ? css`
          border: 1px solid var(--lighter-color);
        `
      : ''}
`;
