import React from "react";
import { Eye, EyeOff, Minimize2, X } from "lucide-react";
import { Button } from "./Button";
import type { WindowControlsProps } from "../types";

export const WindowControls: React.FC<WindowControlsProps> = ({
  isMinimized,
  onToggleMinimize,
  onMinimize,
  onClose,
}) => {
  return (
    <div className="flex items-center gap-1">
      <Button
        variant="ghost"
        size="sm"
        onClick={onToggleMinimize}
        className="p-1"
        title={isMinimized ? "Expand" : "Collapse"}
      >
        {isMinimized ? <Eye size={14} /> : <EyeOff size={14} />}
      </Button>
      <Button
        variant="ghost"
        size="sm"
        onClick={onMinimize}
        className="p-1"
        title="Minimize"
      >
        <Minimize2 size={14} />
      </Button>
      <Button
        variant="ghost"
        size="sm"
        onClick={onClose}
        className="p-1"
        title="Close"
      >
        <X size={14} />
      </Button>
    </div>
  );
};
