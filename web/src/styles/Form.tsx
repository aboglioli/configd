import styled, { css } from 'styled-components';

export interface ButtonProps {
  primary?: boolean;
}

export const Button = styled.button<ButtonProps>`
  align-items: center;
  background-color: ${(props) =>
    props.primary ? 'var(--lighter-color)' : 'var(--lightest-color)'};
  border: 1px solid var(--light-color);
  color: ${(props) => (props.primary ? 'var(--darkest-color)' : 'var(--darker-color)')};
  cursor: pointer;
  display: flex;
  font-size: 1rem;
  gap: 0.25rem;
  padding: 0.5rem 1rem;
  height: 100%;

  svg {
    height: 100%;
    width: auto;
  }

  &:hover {
    border: 1px solid var(--dark-color);
  }
`;

export const Input = styled.input`
  border: 1px solid var(--light-color);
  flex: 1;
  font-family: inherit;
  font-size: 1rem;
  outline: none;
  padding: 0.5rem 1rem;

  &:hover {
    border: 1px solid var(--dark-color);
  }
`;
