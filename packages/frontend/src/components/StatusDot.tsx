import React from "react";
import { cn } from "../utils";

interface StatusDotProps {
  isOnline: boolean;
  size?: "sm" | "md" | "lg";
  className?: string;
}

export const StatusDot: React.FC<StatusDotProps> = ({
  isOnline,
  size = "md",
  className,
}) => {
  const sizeClasses = {
    sm: "w-2 h-2",
    md: "w-3 h-3",
    lg: "w-4 h-4",
  };

  return (
    <div
      className={cn(
        "rounded-full",
        sizeClasses[size],
        isOnline ? "status-dot online" : "status-dot offline",
        className
      )}
    />
  );
};
