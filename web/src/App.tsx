import { useState } from 'react';
import { Routes, Route } from 'react-router-dom';

import Schemas from 'pages/Schemas';
import Schema from 'pages/Schema';
import Config from 'pages/Config';
import Playground from 'pages/Playground';
import { Wrapper } from 'styles/Wrapper';
import { Size } from 'styles/common';
import { Header, HeaderLogo, HeaderTitle, HeaderSubtitle } from 'styles/Header';
import { Link } from 'styles/Link';

import './App.css';

const App = () => {
  const [title, setTitle] = useState('');

  return (
    <>
      <Header>
        <Link to="/">
          <HeaderLogo src="/logo.png" />
        </Link>
        <HeaderTitle>Configd</HeaderTitle>
        {title && <HeaderSubtitle>{title}</HeaderSubtitle>}
      </Header>
      <Wrapper $padding={Size.Large}>
        <Routes>
          <Route path="/" element={<Schemas setTitle={setTitle} />} />
          <Route path="/schemas/:schemaId" element={<Schema setTitle={setTitle} />} />
          <Route
            path="/schemas/:schemaId/configs/:configId"
            element={<Config setTitle={setTitle} />}
          />
          <Route path="/playground" element={<Playground />} />
        </Routes>
      </Wrapper>
    </>
  );
};

export default App;
