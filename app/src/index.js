import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';
import SolanaWalletProvider from './components/solanaWalletAdapter';

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <SolanaWalletProvider>
    <App />
  </SolanaWalletProvider>
);


