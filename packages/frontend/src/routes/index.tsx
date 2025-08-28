import { createFileRoute } from '@tanstack/react-router';
import { LogMonitor } from '../components/log-monitor';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  return (
    <div className="min-h-screen bg-gray-900 text-white p-6">
      <div className="max-w-6xl mx-auto">
        <h1 className="text-4xl font-bold text-center mb-8 text-white">
          The overlord has risen!
        </h1>
        <LogMonitor />
      </div>
    </div>
  );
}
