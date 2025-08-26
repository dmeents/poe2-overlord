import { FooterComponent } from '@/components/footer.component';
import { InfoPanel } from '@/components/info-panel.component';
import { ProcessStatusComponent } from '@/components/process-status.component';
import { QuickActionsComponent } from '@/components/quick-actions.component';
import { usePoe2Process } from '@/hooks/usePoe2Process';
import { createFileRoute } from '@tanstack/react-router';
import { Activity, Info, Search, Target } from 'lucide-react';

export const Route = createFileRoute('/')({
  component: Index,
});

export default function Index() {
  const { poe2Running, processInfo, checkPoe2Process } = usePoe2Process();

  const quickActions = [
    {
      icon: <Search size={16} />,
      label: 'Search',
      onClick: () => console.log('Search clicked'),
    },
    {
      icon: <Target size={16} />,
      label: 'Track',
      onClick: () => console.log('Track clicked'),
    },
  ];

  const infoPanels = [
    {
      title: 'Process Monitor',
      description:
        'Real-time monitoring of Path of Exile 2 process status and resource usage.',
      icon: <Activity size={16} />,
    },
    {
      title: 'Game Overlay',
      description:
        'Built with Tauri and React for optimal performance and cross-platform compatibility.',
      icon: <Info size={16} />,
    },
  ];

  return (
    <div>
      <div>
        <div>
          <div>
            <h1>POE2 Overlord</h1>
          </div>
          <div>
            <span>Process Status: {poe2Running ? 'Running' : 'Not Found'}</span>
          </div>
        </div>
      </div>
      <div>
        <ProcessStatusComponent
          processInfo={processInfo}
          onRefresh={checkPoe2Process}
        />
        <InfoPanel {...infoPanels[0]} />
        <QuickActionsComponent actions={quickActions} />
        <InfoPanel {...infoPanels[1]} />
      </div>
      <FooterComponent />
    </div>
  );
}
