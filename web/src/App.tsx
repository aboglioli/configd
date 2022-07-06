import { useState } from 'react';
import { Routes, Route } from 'react-router-dom';

import Schemas from 'pages/Schemas';
import { Header } from 'components/Header';

import './App.css';

const App = () => {
  const [title, setTitle] = useState('');

  return (
    <>
      <Header title={title} />
      <div className="main-container">
        <Routes>
          <Route path="/" element={<Schemas setTitle={setTitle} />} />
        </Routes>
      </div>
    </>
  );
};

export default App;
