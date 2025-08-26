import { Footer } from '@/components/footer.tsx';
import { InfoPanel } from '@/components/info-panel.tsx';
import { ProcessStatus } from '@/components/process-status.tsx';
import { QuickActions } from '@/components/quick-actions.tsx';
import { usePoe2Process } from '@/hooks/usePoe2Process';
import { createFileRoute } from '@tanstack/react-router';
import { Activity, Info, Search, Target } from 'lucide-react';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
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
        <ProcessStatus processInfo={processInfo} onRefresh={checkPoe2Process} />
        <InfoPanel {...infoPanels[0]} />
        <QuickActions actions={quickActions} />
        <InfoPanel {...infoPanels[1]} />
      </div>
      <Footer />
    </div>
  );
}
