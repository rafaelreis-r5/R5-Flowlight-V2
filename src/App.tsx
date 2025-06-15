import * as React from 'react';
import { useState, useEffect } from 'react';
import { getCurrent } from '@tauri-apps/api/window';
import { SettingsProvider } from './contexts/SettingsContext';
import { AuthProvider } from './contexts/AuthContext';
import ErrorBoundary from './components/ErrorBoundary';

// Import components
import MainWindow from './components/Windows/MainWindow';
import SearchWindow from './components/Windows/SearchWindow';

// Main App component
const App: React.FC = () => {
  const [windowLabel, setWindowLabel] = useState<string | null>(null);

  useEffect(() => {
    const detectWindow = async () => {
      try {
        const currentWindow = getCurrent();
        const label = currentWindow.label;
        setWindowLabel(label);
        console.log(`R5 Flowlight: Rendering window "${label}"`);

        // Add class to body and root for search window
        if (label === 'search') {
          document.body.classList.add('launcher-window');
          document.body.style.background = 'transparent';
          document.body.style.backgroundColor = 'transparent';
          document.documentElement.style.background = 'transparent';
          document.documentElement.style.backgroundColor = 'transparent';
          const root = document.getElementById('root');
          if (root) {
            root.classList.add('launcher-window');
            root.style.background = 'transparent';
            root.style.backgroundColor = 'transparent';
          }
        }
      } catch (error) {
        console.error('Error detecting window:', error);
        setWindowLabel('main');
      }
    };

    detectWindow();
  }, []);

  // Render search window (the floating bar)
  if (windowLabel === 'search') {
    return <SearchWindow />;
  }

  // Render main window
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
