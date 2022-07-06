import { FC } from 'react';

import './Header.css';

export interface HeaderProps {
  title?: string;
}

export const Header: FC<HeaderProps> = ({ title }) => {
  return (
    <header className="header">
      <div className="header__logo">
        <img src="/logo.png" />
      </div>
      <h1 className="header__title">Configd</h1>
      {title && <h2 className="header__subtitle">{title}</h2>}
    </header>
  );
};
