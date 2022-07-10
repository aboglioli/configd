import styled from 'styled-components';

import { Title, Subtitle } from 'styles/Title';

export const HeaderLogo = styled.img`
  height: 32px;
  height: 32px;
  margin-right: 0.5rem;
`;

export const HeaderTitle = styled(Title)`
  font-size: 1.5rem;
`;

export const HeaderSubtitle = styled(Subtitle)`
  border-left: 1px solid #ccc;
  font-size: 1.3rem;
  margin-left: 0.5rem;
  padding-left: 0.5rem;
`;

export const Header = styled.header`
  align-items: center;
  background-color: #eee;
  display: flex;
  padding: 0.5rem 1rem;
  border-bottom: 1px solid #ddd;
`;
