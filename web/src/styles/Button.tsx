import styled from 'styled-components';

export interface ButtonProps {
  primary?: boolean;
}

export const Button = styled.button<ButtonProps>`
  background-color: ${(props) => (props.primary ? '#ddd' : '#eee')};
  border: 1px solid rgba(0, 0, 0, 0.2);
  color: ${(props) => (props.primary ? '#444' : '#666')};
  cursor: pointer;
  font-family: inherit;
  font-size: 1rem;
  padding: 0.25rem 1em;

  &:hover {
    border: 1px solid rgba(0, 0, 0, 0.5);
  }
`;
