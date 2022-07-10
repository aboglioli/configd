import styled, { css } from 'styled-components';

import { Wrapper, VerticalWrapper } from 'styles/Wrapper';

export const List = VerticalWrapper;

export const ListItem = styled(Wrapper)`
  border: 1px solid #eee;
  display: flex;
  gap: 0.5rem;
  align-items: center;
  ${(props) =>
    props.onClick
      ? css`
          cursor: pointer;
        `
      : ''}
`;

export const ListItemImage = styled.img`
  max-width: 32px;
  max-height: 32px;
`;

export const ListItemContent = styled.div`
  flex: 1;
`;

export const ListItemButtons = styled.div`
  padding: 0;
  display: flex;
  gap: 0.5rem;
`;
