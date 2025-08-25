import React from "react";
import { Settings } from "lucide-react";
import { AlertCircle } from "lucide-react";
import {
  Button,
  Footer,
  InfoPanel,
  ProcessStatus,
  QuickActions,
  TitleBar,
} from "./components";
import { usePoe2Process, useWindowControls } from "./hooks";
import { APP_CONFIG } from "./utils";

function App() {
  const { processInfo, poe2Running, checkPoe2Process } = usePoe2Process();
  const { isMinimized, toggleMinimize, minimizeWindow, closeWindow } =
    useWindowControls();

  const windowControls = {
    isMinimized,
    onToggleMinimize: toggleMinimize,
    onMinimize: minimizeWindow,
    onClose: closeWindow,
  };

  return (
    <div className="w-full h-full flex flex-col font-mono bg-gray-900">
      {/* Main Overlay Container */}
      <div className="bg-gray-900/95 fade-in m-2 flex-1 flex flex-col text-white rounded-lg">
        {/* Title Bar */}
        <TitleBar
          poe2Running={poe2Running}
          processInfo={processInfo}
          windowControls={windowControls}
        />

        {/* Content Area */}
        {!isMinimized && (
          <div className="flex-1 p-3 flex flex-col gap-3">
            {/* Process Status */}
            <ProcessStatus
              poe2Running={poe2Running}
              processInfo={processInfo}
              onRefresh={checkPoe2Process}
            />

            {/* Quick Actions */}
            <QuickActions />

            {/* Info Panel */}
            <InfoPanel
              title="Overlay Ready"
              description="The overlay is now active and monitoring for Path of Exile 2. Use the controls above to interact with game data and settings."
              icon={<AlertCircle size={16} />}
            />

            {/* Settings Button */}
            <Button
              variant="primary"
              size="md"
              className="w-full flex items-center justify-center gap-2"
            >
              <Settings size={16} />
              <span>Settings</span>
            </Button>
          </div>
        )}

        {/* Footer */}
        <Footer
          version={APP_CONFIG.VERSION}
          technology={APP_CONFIG.TECHNOLOGY}
        />
      </div>
    </div>
  );
}

export default App;
