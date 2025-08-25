import React from "react";
import { StatusDot } from "./StatusDot";
import { WindowControls } from "./WindowControls";
import { APP_CONFIG } from "../utils";
import type { TitleBarProps } from "../types";

export const TitleBar: React.FC<TitleBarProps> = ({
  poe2Running,
  processInfo,
  windowControls,
}) => {
  return (
    <div className="flex items-center justify-between p-3 border-b border-gray-600">
      <div className="flex items-center gap-2">
        <StatusDot isOnline={poe2Running} size="sm" />
        <h1 className="text-white text-sm font-bold m-0">{APP_CONFIG.TITLE}</h1>
        {poe2Running && processInfo && (
          <span className="text-gray-400 text-xs">PID: {processInfo.pid}</span>
        )}
      </div>

      <WindowControls {...windowControls} />
    </div>
  );
};
