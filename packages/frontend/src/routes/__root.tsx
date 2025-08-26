import { StatusBar, WindowTitle } from '@/components';
import { createRootRoute, Outlet } from '@tanstack/react-router';
import '../globals.css';

export const Route = createRootRoute({
  component: () => (
    <div className='bg-zinc-900 h-screen overflow-hidden'>
      <WindowTitle />
      <div className='h-full mt-[30px] overflow-auto font-sans'>
        <Outlet />
        <StatusBar />
      </div>
    </div>
  ),
});
