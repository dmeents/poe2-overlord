import { createRootRoute, Outlet } from '@tanstack/react-router';
import { SidebarNavigation } from '@/components/layout/sidebar-navigation/sidebar-navigation';
import { WindowTitle } from '@/components/layout/window-title/window-title';
import { StatusBar } from '@/components/status/status-bar/status-bar';
import { ZoneDetailsModal } from '@/components/zones/zone-details-modal/zone-details-modal';
import '../globals.css';

export const Route = createRootRoute({
  component: () => (
    <div className="app-background h-screen overflow-hidden">
      <WindowTitle />
      <SidebarNavigation />
      <div className="h-full mt-[30px] ml-12 overflow-auto font-sans">
        <div className="mb-16">
          <Outlet />
        </div>
      </div>
      <StatusBar />
      <ZoneDetailsModal />
    </div>
  ),
});
