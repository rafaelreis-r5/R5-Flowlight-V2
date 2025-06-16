import React from 'react';
import ReactDOM from 'react-dom/client';
import OverlaySearchWindow from './components/Windows/OverlaySearchWindow';
import { Toaster } from 'react-hot-toast';
import './index.css';

const OverlayApp: React.FC = () => {
  // Aplicar classe CSS para overlay transparente
  React.useEffect(() => {
    document.body.classList.add('overlay-window');
    document.documentElement.classList.add('overlay-window');
    return () => {
      document.body.classList.remove('overlay-window');
      document.documentElement.classList.remove('overlay-window');
    };
  }, []);

  return (
    <div className="overlay-app" style={{
      width: '100vw',
      height: '100vh',
      display: 'flex',
      justifyContent: 'center',
      alignItems: 'flex-start',
      paddingTop: '80px',
      background: 'transparent',
      pointerEvents: 'none', // Permitir cliques passarem através das áreas vazias
      position: 'fixed',
      top: 0,
      left: 0,
      zIndex: 9999
    }}>
      <OverlaySearchWindow />
      <Toaster 
        position="top-center"
        toastOptions={{
          duration: 3000,
          style: {
            background: 'rgba(20, 20, 30, 0.95)',
            color: '#fff',
            border: '1px solid rgba(255, 255, 255, 0.1)',
            borderRadius: '8px',
            backdropFilter: 'blur(20px)',
          },
        }}
      />
    </div>
  );
};

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <OverlayApp />
  </React.StrictMode>
);