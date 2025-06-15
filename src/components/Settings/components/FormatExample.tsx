import React from 'react';
import { useFormatters } from '../../../hooks/useFormatters';

interface FormatExampleProps {
  type: 'date' | 'number' | 'currency';
  value?: string | number | Date;
  currency?: string;
}

export const FormatExample: React.FC<FormatExampleProps> = ({
  type,
  value,
  currency = 'BRL',
}) => {
  const { formatDate, formatNumber, formatCurrency } = useFormatters();
  
  const getFormattedValue = () => {
    const val = value !== undefined ? value : 
      type === 'date' ? new Date() : 
      type === 'currency' ? 1234.56 : 1234.56;
    
    switch (type) {
      case 'date':
        return formatDate(val as Date);
      case 'number':
        return formatNumber(Number(val));
      case 'currency':
        return formatCurrency(Number(val), currency);
      default:
        return String(val);
    }
  };

  return (
    <div className="mt-1 text-xs text-gray-400">
      Exemplo: <span className="font-mono">{getFormattedValue()}</span>
    </div>
  );
};
