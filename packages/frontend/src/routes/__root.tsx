import { createRootRoute, Outlet, useLocation } from '@tanstack/react-router';
import { useEffect, useRef } from 'react';
import { SidebarNavigation } from '@/components/layout/sidebar-navigation/sidebar-navigation';
import { WindowTitle } from '@/components/layout/window-title/window-title';
import { StatusBar } from '@/components/status/status-bar/status-bar';
import { ZoneDetailsModal } from '@/components/zones/zone-details-modal/zone-details-modal';
import { useConfiguration } from '@/contexts/ConfigurationContext';
import { BACKGROUND_IMAGE_CSS } from '@/utils/background-images';
import '../globals.css';

function ScrollToTop({ containerRef }: { containerRef: React.RefObject<HTMLElement | null> }) {
  const { pathname } = useLocation();
  // biome-ignore lint/correctness/useExhaustiveDependencies: containerRef.current is intentionally accessed inside the effect; containerRef itself is stable
  useEffect(() => {
    containerRef.current?.scrollTo(0, 0);
  }, [pathname, containerRef]);
  return null;
}

function RootComponent() {
  const mainRef = useRef<HTMLElement>(null);
  const { config } = useConfiguration();

  const bgCss = config?.background_image
    ? (BACKGROUND_IMAGE_CSS[config.background_image] ?? BACKGROUND_IMAGE_CSS.VolcanicRuins)
    : BACKGROUND_IMAGE_CSS.VolcanicRuins;

  return (
    <div className="app-background" style={{ '--bg-image': bgCss } as React.CSSProperties}>
      <WindowTitle />
      <SidebarNavigation />
      <main
        ref={mainRef}
        className="relative h-[calc(100vh-52px)] mt-[28px] ml-12 overflow-auto font-sans">
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
