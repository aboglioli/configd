import styled, { css } from 'styled-components';

export interface MessageProps {
  $info?: boolean;
  $success?: boolean;
  $warning?: boolean;
  $error?: boolean;
}

export const Message = styled.div<MessageProps>`
  border: 1px solid rgba(0, 0, 0, 0.1);
  padding: 0.5rem 1rem;

  ${(props) =>
    props.$info
      ? css`
          background-color: #d6fbff;
        `
      : props.$success
      ? css`
          background-color: #dcffdc;
        `
      : props.$warning
      ? css`
          background-color: #fbead0;
        `
      : props.$error
      ? css`
          background-color: #ffe0e0;
        `
      : css`
          background-color: var(--lightest-color);
        `}
`;
