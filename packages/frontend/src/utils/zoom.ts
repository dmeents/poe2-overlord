import { getCurrentWebview } from '@tauri-apps/api/webview';
import { currentMonitor, getCurrentWindow } from '@tauri-apps/api/window';

export const ZOOM_OPTIONS = [
  { value: 0, label: 'Auto' },
  { value: 0.75, label: '75%' },
  { value: 1.0, label: '100%' },
  { value: 1.25, label: '125%' },
  { value: 1.5, label: '150%' },
  { value: 1.75, label: '175%' },
  { value: 2.0, label: '200%' },
];

async function computeAutoZoom(): Promise<number> {
  const win = getCurrentWindow();
  const scaleFactor = await win.scaleFactor();

  // If the OS already applies scaling (e.g. Hyprland scale > 1), don't double-scale
  if (scaleFactor > 1.05) return 1.0;

  const monitor = await currentMonitor();
  if (!monitor) return 1.0;

  // Logical width = physical pixels / OS scale factor
  const logicalWidth = monitor.size.width / scaleFactor;

  if (logicalWidth > 5000) return 2.0;
  if (logicalWidth > 3800) return 1.75;
  if (logicalWidth > 3000) return 1.5;
  if (logicalWidth > 2500) return 1.25;
  return 1.0;
}

export async function applyZoom(level: number): Promise<void> {
  const effectiveZoom = level === 0 ? await computeAutoZoom() : level;
  await getCurrentWebview().setZoom(effectiveZoom);
}
