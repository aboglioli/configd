import { useState } from 'react';
import { Routes, Route } from 'react-router-dom';

import Schemas from 'pages/Schemas';
import Playground from 'pages/Playground';
import { Wrapper } from 'styles/Wrapper';
import { Size } from 'styles/common';
import { Header, HeaderLogo, HeaderTitle, HeaderSubtitle } from 'styles/Header';

import './App.css';

const App = () => {
  const [title, setTitle] = useState('');

  return (
    <>
      <Header>
        <HeaderLogo src="/logo.png" />
        <HeaderTitle>Configd</HeaderTitle>
        {title && <HeaderSubtitle>{title}</HeaderSubtitle>}
      </Header>
      <Wrapper padding={Size.Large}>
        <Routes>
          <Route path="/" element={<Schemas setTitle={setTitle} />} />
          <Route path="/playground" element={<Playground />} />
        </Routes>
      </Wrapper>
    </>
  );
};

export default App;
