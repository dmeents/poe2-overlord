import type { InfoPanelProps } from '@/types';
import React from 'react';

export const InfoPanel: React.FC<InfoPanelProps> = ({
  title,
  description,
  icon,
}) => {
  return (
    <div>
      <div>
        <div>{icon}</div>
        <div>
          <h3>{title}</h3>
          <p>{description}</p>
        </div>
      </div>
    </div>
  );
};
