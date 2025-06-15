import React from 'react';

export const TestComponent: React.FC = () => {
  console.log('TestComponent está sendo renderizado');
  
  return (
    <div style={{
      position: 'fixed',
      top: '50%',
      left: '50%',
      transform: 'translate(-50%, -50%)',
      backgroundColor: 'red',
      color: 'white',
      padding: '20px',
      borderRadius: '8px',
      zIndex: 9999,
    }}>
      <h1>Teste de Renderização</h1>
      <p>Se você está vendo isso, o React está funcionando!</p>
    </div>
  );
};

export default TestComponent;
