import styled, { css } from 'styled-components';

import { Box } from 'styles/Box';
import { Alignment } from 'styles/common';

export interface WrapperProps {
  $alignment?: Alignment;
  $verticalAlignment?: Alignment;
  $vertical?: boolean;
}

export const Wrapper = styled(Box)<WrapperProps>`
  display: flex;
  flex-wrap: wrap;
  flex: 1;

  ${(props) =>
    props.$alignment
      ? css`
          justify-content: ${props.$alignment};
        `
      : ''}
  ${(props) =>
    props.$verticalAlignment
      ? css`
          align-items: ${props.$verticalAlignment};
        `
      : ''}

      ${(props) =>
    props.$vertical
      ? css`
          flex-direction: column;
        `
      : ''}
`;
