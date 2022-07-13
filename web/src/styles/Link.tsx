import styled, { css } from 'styled-components';
import { Link as ReactRouterLink } from 'react-router-dom';

const link = css`
  text-decoration: none;
  color: var(--darkest-color);

  &:hover {
    text-decoration: underline;
  }
`;

export const ExternalLink = styled.a`
  ${link}
`;

export const Link = styled(ReactRouterLink)`
  ${link}
`;
