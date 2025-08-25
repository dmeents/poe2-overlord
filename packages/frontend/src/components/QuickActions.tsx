import React from "react";
import { Search, Target } from "lucide-react";
import { Button } from "./Button";

const defaultActions = [
  {
    icon: <Search size={16} />,
    label: "Search",
    onClick: () => console.log("Search clicked"),
  },
  {
    icon: <Target size={16} />,
    label: "Track",
    onClick: () => console.log("Track clicked"),
  },
];

export const QuickActions: React.FC = () => {
  return (
    <div className="grid grid-cols-2 gap-2">
      {defaultActions.map((action, index) => (
        <Button
          key={index}
          variant="primary"
          size="md"
          onClick={action.onClick}
          className="flex items-center justify-center gap-2"
        >
          {action.icon}
          <span>{action.label}</span>
        </Button>
      ))}
    </div>
  );
};
