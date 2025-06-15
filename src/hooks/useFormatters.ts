import { useSettings } from '../contexts/SettingsContext';

export const useFormatters = () => {
  const { settings } = useSettings();

  const formatDate = (date: Date | string | number, format: string = settings.general.dateFormat): string => {
    const d = new Date(date);
    if (isNaN(d.getTime())) return 'Data inválida';

    const day = d.getDate().toString().padStart(2, '0');
    const month = (d.getMonth() + 1).toString().padStart(2, '0');
    const year = d.getFullYear();
    const monthName = d.toLocaleString(settings.general.language, { month: 'long' });
    const monthShort = d.toLocaleString(settings.general.language, { month: 'short' });

    return format
      .replace('dd', day)
      .replace('MM', month)
      .replace('MMMM', monthName)
      .replace('MMM', monthShort)
      .replace('yyyy', year.toString())
      .replace('d', d.getDate().toString())
      .replace('D', d.getDate().toString());
  };

  const formatNumber = (value: number, options?: Intl.NumberFormatOptions): string => {
    try {
      return new Intl.NumberFormat(settings.general.numberFormat, options).format(value);
    } catch (error) {
      console.error('Erro ao formatar número:', error);
      return value.toString();
    }
  };

  const formatCurrency = (value: number, currency: string = 'BRL'): string => {
    return formatNumber(value, {
      style: 'currency',
      currency,
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    });
  };

  return {
    formatDate,
    formatNumber,
    formatCurrency,
  };
};
