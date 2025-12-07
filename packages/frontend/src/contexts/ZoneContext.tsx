/* eslint-disable react-refresh/only-export-components */
import type { ZoneStats } from '@/types/character';
import {
  createContext,
  useContext,
  useState,
  useCallback,
  useMemo,
} from 'react';
import { useCharacter } from './CharacterContext';
import { createPlaceholderZone } from '@/utils/zone-utils';

interface ZoneContextValue {
  selectedZone: ZoneStats | null;
  isModalOpen: boolean;
  openZone: (zoneName: string) => void;
  closeModal: () => void;
  navigateToZone: (zone: ZoneStats) => void;
  allZones: ZoneStats[];
}

const ZoneContext = createContext<ZoneContextValue | undefined>(undefined);

export function ZoneProvider({ children }: React.PropsWithChildren) {
  const { activeCharacter } = useCharacter();
  const [selectedZone, setSelectedZone] = useState<ZoneStats | null>(null);
  const [isModalOpen, setIsModalOpen] = useState(false);

  const allZones = useMemo(
    () => activeCharacter?.zones || [],
    [activeCharacter?.zones]
  );

  const openZone = useCallback(
    (zoneName: string) => {
      const zone = allZones.find(z => z.zone_name === zoneName);

      if (zone) setSelectedZone(zone);
      else setSelectedZone(createPlaceholderZone(zoneName));

      setIsModalOpen(true);
    },
    [allZones]
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
    [selectedZone, isModalOpen, openZone, closeModal, navigateToZone, allZones]
  );

  return (
    <ZoneContext.Provider value={contextValue}>{children}</ZoneContext.Provider>
  );
}

export function useZone() {
  const context = useContext(ZoneContext);

  if (context === undefined) {
    throw new Error('useZone must be used within ZoneProvider');
  }

  return context;
}
