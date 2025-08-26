import { WindowTitle } from '@/components';
import { createRootRoute, Outlet } from '@tanstack/react-router';
import '../globals.css';

export const Route = createRootRoute({
  component: () => (
    <>
      <div className='bg-zinc-900 h-screen'>
        <WindowTitle />
        <Outlet />
      </div>
    </>
  ),
});
