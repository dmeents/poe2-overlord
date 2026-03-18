import { createRootRoute, Outlet, useLocation } from '@tanstack/react-router';
import { useEffect, useRef } from 'react';
import { SidebarNavigation } from '@/components/layout/sidebar-navigation/sidebar-navigation';
import { WindowTitle } from '@/components/layout/window-title/window-title';
import { StatusBar } from '@/components/status/status-bar/status-bar';
import { ZoneDetailsModal } from '@/components/zones/zone-details-modal/zone-details-modal';
import '../globals.css';

function ScrollToTop({ containerRef }: { containerRef: React.RefObject<HTMLElement | null> }) {
  const { pathname } = useLocation();
  useEffect(() => {
    containerRef.current?.scrollTo(0, 0);
  }, [pathname, containerRef]);
  return null;
}

function RootComponent() {
  const mainRef = useRef<HTMLElement>(null);
  return (
    <div className="app-background">
      <WindowTitle />
      <SidebarNavigation />
      <main ref={mainRef} className="relative h-[calc(100vh-52px)] mt-[28px] ml-12 overflow-auto font-sans">
        <ScrollToTop containerRef={mainRef} />
        <div className="mb-16">
          <Outlet />
        </div>
      </main>
      <StatusBar />
      <ZoneDetailsModal />
    </div>
  );
}

export const Route = createRootRoute({
  component: RootComponent,
});
