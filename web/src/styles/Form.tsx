import styled, { css } from 'styled-components';
import { Link } from 'react-router-dom';

import { Size } from 'styles/common';

export interface ButtonProps {
  $primary?: boolean;
  $size?: Size;
}

const button = css<ButtonProps>`
  align-items: center;
  background-color: ${(props) =>
    props.$primary ? 'var(--lighter-color)' : 'var(--lightest-color)'};
  border: 1px solid var(--light-color);
  color: ${(props) => (props.$primary ? 'var(--darkest-color)' : 'var(--darker-color)')};
  cursor: pointer;
  display: flex;
  gap: 0.25rem;

  ${(props) =>
    props.$size == Size.Small
      ? css`
          font-size: 0.8rem;
          padding: 0.25rem 0.5rem;
        `
      : props.$size == Size.Large
      ? css`
          font-size: 1.2rem;
          padding: 1rem 1.5rem;
        `
      : css`
          font-size: 1rem;
          padding: 0.5rem 1rem;
        `}

  &:hover {
    border: 1px solid var(--dark-color);
  }
`;

export const Button = styled.button.attrs<ButtonProps>((props) => ({
  $size: props.$size || Size.Medium,
}))`
  ${button}
`;

export const ButtonLink = styled(Link).attrs<ButtonProps>((props) => ({
  $size: props.$size || Size.Small,
}))`
  ${button}
  text-decoration: none;
`;

export interface InputProps {
  $size?: Size;
}

export const Input = styled.input<InputProps>`
  border: 1px solid var(--light-color);
  flex: 1;
  font-family: inherit;
  font-size: 1rem;
  outline: none;
  padding: 0.5rem 1rem;

  ${(props) =>
    props.$size == Size.Small
      ? css`
          font-size: 0.8rem;
          padding: 0.25rem 0.5rem;
        `
      : props.$size == Size.Large
      ? css`
          font-size: 1.2rem;
          padding: 1rem 1.5rem;
        `
      : css`
          font-size: 1rem;
          padding: 0.5rem 1rem;
        `}

  &:hover {
    border: 1px solid var(--dark-color);
  }
`;
