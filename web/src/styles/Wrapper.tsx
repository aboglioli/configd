import styled, { css } from 'styled-components';

import { Box } from 'styles/Box';

export interface WrapperProps {
  justifyContent?: string;
  alignItems?: string;
  highlightOnHover?: boolean;
}

export const Wrapper = styled(Box)<WrapperProps>`
  display: flex;
  flex-wrap: wrap;
  width: 100%;
  ${(props) =>
    props.justifyContent
      ? css`
          justify-content: ${props.justifyContent};
        `
      : ''}
  ${(props) =>
    props.alignItems
      ? css`
          align-items: ${props.alignItems};
        `
      : ''}

  ${(props) =>
    props.highlightOnHover
      ? css`
          &:hover {
            border: 1px solid rgba(0, 0, 0, 0.5);
          }
        `
      : ''}
`;

export const VerticalWrapper = styled(Wrapper)`
  flex-direction: column;
`;
