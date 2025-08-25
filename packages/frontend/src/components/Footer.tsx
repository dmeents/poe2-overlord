import React from "react";
import type { FooterProps } from "../types";

export const Footer: React.FC<FooterProps> = ({ version, technology }) => {
  return (
    <div className="p-2 border-t border-gray-600">
      <div className="flex items-center justify-between text-xs text-gray-400">
        <span>{version}</span>
        <span>{technology}</span>
      </div>
    </div>
  );
};
