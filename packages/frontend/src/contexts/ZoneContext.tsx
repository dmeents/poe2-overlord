/* eslint-disable react-refresh/only-export-components */

import { invoke } from '@tauri-apps/api/core';
import { createContext, useCallback, useContext, useMemo, useState } from 'react';
import { useCharacterZones } from '@/queries/characters';
import type { ZoneMetadata } from '@/queries/zones';
import type { ZoneStats } from '@/types/character';
import { createPlaceholderZone } from '@/utils/zone-utils';
import { useCharacter } from './CharacterContext';

interface ZoneContextValue {
  selectedZone: ZoneStats | null;
  isModalOpen: boolean;
  openZone: (zoneName: string) => Promise<void>;
  closeModal: () => void;
  navigateToZone: (zone: ZoneStats) => void;
  allZones: ZoneStats[];
}

const ZoneContext = createContext<ZoneContextValue | undefined>(undefined);

export function ZoneProvider({ children }: React.PropsWithChildren) {
  const { activeCharacter } = useCharacter();
  const [selectedZone, setSelectedZone] = useState<ZoneStats | null>(null);
  const [isModalOpen, setIsModalOpen] = useState(false);

  const { data: allZones = [] } = useCharacterZones(activeCharacter?.id);

  const openZone = useCallback(
    async (zoneName: string) => {
      const zone = allZones.find(z => z.zone_name === zoneName);

      if (zone) {
        setSelectedZone(zone);
      } else {
        // Look up global wiki metadata so unvisited zones still show descriptions, bosses, etc.
        const metadata = await invoke<ZoneMetadata | null>('get_zone_metadata_by_name', {
          zoneName,
        }).catch(() => null);
        setSelectedZone(createPlaceholderZone(zoneName, metadata ?? undefined));
      }

      setIsModalOpen(true);
    },
    [allZones],
  );

  const closeModal = useCallback(() => {
    setIsModalOpen(false);
  }, []);

  const navigateToZone = useCallback((zone: ZoneStats) => {
    setSelectedZone(zone);
  }, []);

  const contextValue = useMemo(
    () => ({
      selectedZone,
      isModalOpen,
      openZone,
      closeModal,
      navigateToZone,
      allZones,
    }),
    [selectedZone, isModalOpen, openZone, closeModal, navigateToZone, allZones],
  );

  return <ZoneContext.Provider value={contextValue}>{children}</ZoneContext.Provider>;
}

export function useZone() {
  const context = useContext(ZoneContext);

  if (context === undefined) {
    throw new Error('useZone must be used within ZoneProvider');
  }

  return context;
}
