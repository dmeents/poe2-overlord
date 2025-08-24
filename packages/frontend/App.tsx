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
    <div style={{ 
      width: '100%', 
      height: '100%', 
      display: 'flex', 
      flexDirection: 'column', 
      background: 'transparent',
      fontFamily: 'JetBrains Mono, Consolas, Monaco, monospace'
    }}>
      {/* Main Overlay Container */}
      <div className="glass-effect fade-in" style={{ 
        margin: '0.5rem', 
        flex: 1, 
        display: 'flex', 
        flexDirection: 'column',
        color: '#ffffff'
      }}>
        
        {/* Title Bar */}
        <div 
          className="draggable"
          style={{
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            padding: '0.75rem',
            borderBottom: '1px solid #404040'
          }}
          onMouseDown={handleDragStart}
        >
          <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
            <div className={`status-dot ${state.poe2Running ? 'online' : 'offline'}`}></div>
            <h1 style={{ 
              color: '#ffffff', 
              fontSize: '0.875rem', 
              fontWeight: 'bold', 
              margin: 0 
            }}>
              POE2 Master
            </h1>
            {state.poe2Running && state.processInfo && (
              <span style={{ color: '#888888', fontSize: '0.75rem' }}>
                PID: {state.processInfo.pid}
              </span>
            )}
          </div>
          
          <div style={{ display: 'flex', alignItems: 'center', gap: '0.25rem' }}>
            <button
              onClick={() => setState(prev => ({ ...prev, isMinimized: !prev.isMinimized }))}
              style={{
                padding: '0.25rem',
                background: 'none',
                border: 'none',
                borderRadius: '0.25rem',
                cursor: 'pointer',
                color: '#cccccc',
                display: 'flex',
                alignItems: 'center'
              }}
              title={state.isMinimized ? "Expand" : "Collapse"}
            >
              {state.isMinimized ? <Eye size={14} /> : <EyeOff size={14} />}
            </button>
            <button
              onClick={minimizeWindow}
              style={{
                padding: '0.25rem',
                background: 'none',
                border: 'none',
                borderRadius: '0.25rem',
                cursor: 'pointer',
                color: '#cccccc',
                display: 'flex',
                alignItems: 'center'
              }}
              title="Minimize"
            >
              <Minimize2 size={14} />
            </button>
            <button
              onClick={closeWindow}
              style={{
                padding: '0.25rem',
                background: 'none',
                border: 'none',
                borderRadius: '0.25rem',
                cursor: 'pointer',
                color: '#cccccc',
                display: 'flex',
                alignItems: 'center'
              }}
              title="Close"
            >
              <X size={14} />
            </button>
          </div>
        </div>

        {/* Content Area */}
        {!state.isMinimized && (
          <div style={{ flex: 1, padding: '0.75rem', display: 'flex', flexDirection: 'column', gap: '0.75rem' }}>
            
            {/* Process Status */}
            <div style={{ 
              backgroundColor: '#1a1a1a', 
              borderRadius: '0.375rem', 
              padding: '0.75rem', 
              border: '1px solid #404040' 
            }}>
              <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                  <Activity 
                    size={16} 
                    color={state.poe2Running ? "#28a745" : "#dc3545"} 
                  />
                  <span style={{ color: '#ffffff', fontSize: '0.875rem' }}>
                    Path of Exile 2
                  </span>
                </div>
                <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                  <span style={{
                    fontSize: '0.75rem',
                    padding: '0.25rem 0.5rem',
                    borderRadius: '0.25rem',
                    backgroundColor: state.poe2Running ? 'rgba(40, 167, 69, 0.2)' : 'rgba(220, 53, 69, 0.2)',
                    color: state.poe2Running ? '#28a745' : '#dc3545'
                  }}>
                    {state.poe2Running ? 'Running' : 'Not Found'}
                  </span>
                  <button
                    onClick={checkPoe2Process}
                    className="btn-poe"
                    style={{ fontSize: '0.75rem' }}
                    title="Refresh Process Status"
                  >
                    Refresh
                  </button>
                </div>
              </div>
              
              {state.processInfo && (
                <div style={{ marginTop: '0.5rem', color: '#888888', fontSize: '0.75rem' }}>
                  Process: {state.processInfo.name}
                </div>
              )}
            </div>

            {/* Quick Actions */}
            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '0.5rem' }}>
              <button className="btn-poe" style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', gap: '0.5rem' }}>
                <Search size={16} />
                <span>Search</span>
              </button>
              <button className="btn-poe" style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', gap: '0.5rem' }}>
                <Target size={16} />
                <span>Track</span>
              </button>
            </div>

            {/* Info Panel */}
            <div style={{ 
              backgroundColor: '#1a1a1a', 
              borderRadius: '0.375rem', 
              padding: '0.75rem', 
              border: '1px solid #404040' 
            }}>
              <div style={{ display: 'flex', alignItems: 'flex-start', gap: '0.5rem' }}>
                <AlertCircle size={16} color="#17a2b8" style={{ marginTop: '0.125rem', flexShrink: 0 }} />
                <div>
                  <h3 style={{ 
                    color: '#ffffff', 
                    fontSize: '0.875rem', 
                    fontWeight: '600',
                    margin: '0 0 0.25rem 0'
                  }}>
                    Overlay Ready
                  </h3>
                  <p style={{ 
                    color: '#888888', 
                    fontSize: '0.75rem', 
                    margin: 0, 
                    lineHeight: 1.4 
                  }}>
                    The overlay is now active and monitoring for Path of Exile 2. 
                    Use the controls above to interact with game data and settings.
                  </p>
                </div>
              </div>
            </div>

            {/* Settings Button */}
            <button className="btn-poe" style={{ 
              width: '100%', 
              display: 'flex', 
              alignItems: 'center', 
              justifyContent: 'center', 
              gap: '0.5rem' 
            }}>
              <Settings size={16} />
              <span>Settings</span>
            </button>
          </div>
        )}
        
        {/* Footer */}
        <div style={{ 
          padding: '0.5rem', 
          borderTop: '1px solid #404040' 
        }}>
          <div style={{ 
            display: 'flex', 
            alignItems: 'center', 
            justifyContent: 'space-between', 
            fontSize: '0.75rem', 
            color: '#888888' 
          }}>
            <span>v0.1.0</span>
            <span>Tauri + React</span>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
