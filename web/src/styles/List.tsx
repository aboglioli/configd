import styled, { css } from 'styled-components';

import { Wrapper } from 'styles/Wrapper';

export const ListItem = styled(Wrapper)`
  align-items: center;
  border: 1px solid var(--lightest-color);
  display: flex;
  gap: 0.5rem;
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
