import styled from 'styled-components';

export const Card = styled.div`
  background-color: var(--fourth-color);
  border: 2px solid var(--first-color);
  border-radius: 3px;
  padding: 1rem;
`;

export const CardTitle = styled.div`
  color: var(--first-color);
`;

export const CardSubtitle = styled.div`
  color: var(--second-color);
`;

export const CardContent = styled.div`
  color: #333;
  font-size: 1rem;
  padding: 1.2rem 0;
`;

export const CardFooter = styled.div`
  align-items: center;
  color: var(--second-color);
  display: flex;
  font-size: 0.8rem;
  justify-content: space-between;
  padding: 0.5rem 0;
`;
