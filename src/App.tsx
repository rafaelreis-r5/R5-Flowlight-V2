import * as React from 'react';
import { useState, useEffect } from 'react';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { SettingsProvider } from './contexts/SettingsContext';
import { AuthProvider } from './contexts/AuthContext';
import ErrorBoundary from './components/ErrorBoundary';

// Import components
import MainWindow from './components/Windows/MainWindow';

// Main App component
const App: React.FC = () => {
  return (
    <div className="app-container">
      <style>{`
        .app-container {
          width: 100vw;
          height: 100vh;
          display: flex;
          flex-direction: column;
          background-color: #0f0e17;
          color: white;
          overflow: hidden;
        }

        /* Global scrollbar styles */
        ::-webkit-scrollbar {
          width: 8px;
          height: 8px;
        }

        ::-webkit-scrollbar-track {
          background: rgba(9, 0, 0, 0.1);
          border-radius: 4px;
        }

        ::-webkit-scrollbar-thumb {
          background: rgba(255, 255, 255, 0.2);
          border-radius: 4px;
        }

        ::-webkit-scrollbar-thumb:hover {
          background: rgba(255, 255, 255, 0.3);
        }
      `}</style>
      <ErrorBoundary>
        <AuthProvider>
          <SettingsProvider>
            <MainWindow />
          </SettingsProvider>
        </AuthProvider>
      </ErrorBoundary>
    </div>
  );
};

export default App;
