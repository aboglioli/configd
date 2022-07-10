import styled from 'styled-components';

export interface ButtonProps {
  primary?: boolean;
}

export const Button = styled.button<ButtonProps>`
  background-color: ${(props) =>
    props.primary ? 'var(--lighter-color)' : 'var(--lightest-color)'};
  border: 1px solid var(--light-color);
  color: ${(props) => (props.primary ? 'var(--darkest-color)' : 'var(--darker-color)')};
  cursor: pointer;
  font-family: inherit;
  font-size: 1rem;
  padding: 0.25rem 1em;

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
  padding: 0.5rem;

  &:hover {
    border: 1px solid var(--dark-color);
  }
`;
