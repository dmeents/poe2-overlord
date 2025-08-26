import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  return (
    <div className='text-2xl font-bold text-center text-white'>
      The overlord has risen!
    </div>
  );
}
