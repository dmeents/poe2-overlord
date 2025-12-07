import { SidebarNavigation } from '@/components/layout/sidebar-navigation/sidebar-navigation';
import { StatusBar } from '@/components/status/status-bar/status-bar';
import { WindowTitle } from '@/components/layout/window-title/window-title';
import { ZoneDetailsModal } from '@/components/zones/zone-details-modal/zone-details-modal';
import { createRootRoute, Outlet } from '@tanstack/react-router';
import '../globals.css';

export const Route = createRootRoute({
  component: () => (
    <div className='bg-zinc-900 h-screen overflow-hidden'>
      <WindowTitle />
      <SidebarNavigation />
      <div className='h-full mt-[30px] ml-12 overflow-auto font-sans'>
        <div className='mb-16'>
          <Outlet />
        </div>
      </div>
      <StatusBar />
      <ZoneDetailsModal />
    </div>
  ),
});
