import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { 
  Settings, 
  Eye, 
  EyeOff, 
  Minimize2, 
  X, 
  Activity,
  AlertCircle,
  Search,
  Target
} from 'lucide-react';
import './App.css';

interface ProcessInfo {
  name: string;
  pid: number;
  running: boolean;
}

interface OverlayState {
  visible: boolean;
  poe2Running: boolean;
  processInfo: ProcessInfo | null;
  isDragging: boolean;
  isMinimized: boolean;
}

function App() {
  const [state, setState] = useState<OverlayState>({
    visible: true,
    poe2Running: false,
    processInfo: null,
    isDragging: false,
    isMinimized: false,
  });

  useEffect(() => {
    // Listen for POE2 process status updates from Rust backend
    const unsubscribe = listen<ProcessInfo>('poe2-process-status', (event) => {
      setState(prev => ({
        ...prev,
        poe2Running: event.payload.running,
        processInfo: event.payload,
      }));
    });

    // Initial process check
    checkPoe2Process();

    return () => {
      unsubscribe.then(fn => fn());
    };
  }, []);

  const checkPoe2Process = async () => {
    try {
      const processInfo = await invoke<ProcessInfo>('check_poe2_process');
      setState(prev => ({
        ...prev,
        poe2Running: processInfo.running,
        processInfo,
      }));
    } catch (error) {
      console.error('Failed to check POE2 process:', error);
    }
  };

  const minimizeWindow = async () => {
    try {
      const window = getCurrentWindow();
      await window.minimize();
      setState(prev => ({ ...prev, isMinimized: true }));
    } catch (error) {
      console.error('Failed to minimize window:', error);
    }
  };

  const closeWindow = async () => {
    try {
      const window = getCurrentWindow();
      await window.close();
    } catch (error) {
      console.error('Failed to close window:', error);
    }
  };

  const handleDragStart = async () => {
    try {
      setState(prev => ({ ...prev, isDragging: true }));
      const window = getCurrentWindow();
      await window.startDragging();
    } catch (error) {
      console.error('Failed to start dragging:', error);
    } finally {
      setState(prev => ({ ...prev, isDragging: false }));
    }
  };

  return (
    <div style={{ width: '100%', height: '100%', display: 'flex', flexDirection: 'column', background: 'transparent' }}>
      {/* Main Overlay Container */}
      <div className="glass-effect fade-in" style={{ margin: '0.5rem', flex: 1, display: 'flex', flexDirection: 'column' }}>
        
        {/* Title Bar */}
        <div 
          className="draggable flex items-center justify-between p-3 border-b border-poe-border-primary"
          onMouseDown={handleDragStart}
        >
          <div className="flex items-center space-x-2">
            <div className={`status-dot ${state.poe2Running ? 'online' : 'offline'}`}></div>
            <h1 className="text-poe-text-primary font-mono text-sm font-bold">
              POE2 Master
            </h1>
            {state.poe2Running && state.processInfo && (
              <span className="text-poe-text-muted text-xs">
                PID: {state.processInfo.pid}
              </span>
            )}
          </div>
          
          <div className="flex items-center space-x-1">
            <button
              onClick={() => setState(prev => ({ ...prev, isMinimized: !prev.isMinimized }))}
              className="p-1 hover:bg-poe-bg-tertiary rounded transition-colors"
              title={state.isMinimized ? "Expand" : "Collapse"}
            >
              {state.isMinimized ? (
                <Eye size={14} className="text-poe-text-secondary" />
              ) : (
                <EyeOff size={14} className="text-poe-text-secondary" />
              )}
            </button>
            <button
              onClick={minimizeWindow}
              className="p-1 hover:bg-poe-bg-tertiary rounded transition-colors"
              title="Minimize"
            >
              <Minimize2 size={14} className="text-poe-text-secondary" />
            </button>
            <button
              onClick={closeWindow}
              className="p-1 hover:bg-poe-accent-danger rounded transition-colors"
              title="Close"
            >
              <X size={14} className="text-poe-text-secondary hover:text-white" />
            </button>
          </div>
        </div>

        {/* Content Area */}
        {!state.isMinimized && (
          <div className="flex-1 p-3 space-y-3">
            
            {/* Process Status */}
            <div className="bg-poe-bg-secondary rounded-md p-3 border border-poe-border-primary">
              <div className="flex items-center justify-between">
                <div className="flex items-center space-x-2">
                  <Activity size={16} className={state.poe2Running ? "text-poe-accent-success" : "text-poe-accent-danger"} />
                  <span className="text-poe-text-primary font-mono text-sm">
                    Path of Exile 2
                  </span>
                </div>
                <div className="flex items-center space-x-2">
                  <span className={`text-xs px-2 py-1 rounded ${
                    state.poe2Running 
                      ? 'bg-poe-accent-success/20 text-poe-accent-success' 
                      : 'bg-poe-accent-danger/20 text-poe-accent-danger'
                  }`}>
                    {state.poe2Running ? 'Running' : 'Not Found'}
                  </span>
                  <button
                    onClick={checkPoe2Process}
                    className="btn-poe text-xs"
                    title="Refresh Process Status"
                  >
                    Refresh
                  </button>
                </div>
              </div>
              
              {state.processInfo && (
                <div className="mt-2 text-poe-text-muted text-xs font-mono">
                  Process: {state.processInfo.name}
                </div>
              )}
            </div>

            {/* Quick Actions */}
            <div className="grid grid-cols-2 gap-2">
              <button className="btn-poe flex items-center justify-center space-x-2 p-3">
                <Search size={16} />
                <span>Search</span>
              </button>
              <button className="btn-poe flex items-center justify-center space-x-2 p-3">
                <Target size={16} />
                <span>Track</span>
              </button>
            </div>

            {/* Info Panel */}
            <div className="bg-poe-bg-secondary rounded-md p-3 border border-poe-border-primary">
              <div className="flex items-start space-x-2">
                <AlertCircle size={16} className="text-poe-accent-info mt-0.5" />
                <div>
                  <h3 className="text-poe-text-primary font-mono text-sm font-semibold">
                    Overlay Ready
                  </h3>
                  <p className="text-poe-text-muted text-xs mt-1">
                    The overlay is now active and monitoring for Path of Exile 2. 
                    Use the controls above to interact with game data and settings.
                  </p>
                </div>
              </div>
            </div>

            {/* Settings Button */}
            <button className="btn-poe w-full flex items-center justify-center space-x-2 p-3">
              <Settings size={16} />
              <span>Settings</span>
            </button>
          </div>
        )}
        
        {/* Footer */}
        <div className="p-2 border-t border-poe-border-primary">
          <div className="flex items-center justify-between text-xs text-poe-text-muted font-mono">
            <span>v0.1.0</span>
            <span>Tauri + React</span>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
