import type { ReactNode } from 'react';
import { dataItemStyles } from './data-item.styles';

export interface DataItemProps {
  label: string | ReactNode;
  value: string | number;
  subValue?: string;
  className?: string;
  color?: string;
  icon?: ReactNode;
}

export function DataItem({ label, value, subValue, className = '', color, icon }: DataItemProps) {
  return (
    <div
      className={`${dataItemStyles.container} ${className}`}
      style={color ? { borderLeftColor: color } : undefined}>
      <div className={dataItemStyles.labelContainer}>
        {icon && <div className={dataItemStyles.icon}>{icon}</div>}
        <span className={dataItemStyles.label}>{label}</span>
      </div>
      <div className={dataItemStyles.valueContainer}>
        <div className={dataItemStyles.value}>{value}</div>
        {subValue && <div className={dataItemStyles.subValue}>{subValue}</div>}
      </div>
    </div>
  );
}
