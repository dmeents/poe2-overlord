import { UsersIcon, InboxIcon } from '@heroicons/react/24/outline';
import { useMemo } from 'react';
import type { CharacterData } from '../../../types/character';
import { ClassDistributionChart } from '../../charts/class-distribution-chart/class-distribution-chart';
import { DataCard } from '../../ui/data-card/data-card';

interface ClassDistributionInsightsProps {
  characters: CharacterData[];
}

export function ClassDistributionInsights({
  characters,
}: ClassDistributionInsightsProps) {
  const classDistribution = useMemo(() => {
    return characters.reduce(
      (acc, c) => {
        acc[c.class] = (acc[c.class] || 0) + 1;
        return acc;
      },
      {} as Record<string, number>
    );
  }, [characters]);

  if (characters.length === 0 || Object.keys(classDistribution).length === 0) {
    return (
      <DataCard
        title='Class Distribution'
        icon={<UsersIcon className='w-5 h-5' />}
        isEmpty={true}
        emptyTitle='No Class Data'
        emptyDescription='Create characters to see class distribution'
        emptyIcon={<InboxIcon className='w-8 h-8' />}
      >
        <div></div>
      </DataCard>
    );
  }

  return (
    <DataCard
      title='Class Distribution'
      icon={<UsersIcon className='w-5 h-5' />}
    >
      <ClassDistributionChart classDistribution={classDistribution} />
    </DataCard>
  );
}
