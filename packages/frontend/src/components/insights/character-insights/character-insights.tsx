import {
  BoltIcon,
  ChartBarIcon,
  GlobeAltIcon,
  InboxIcon,
  UserGroupIcon,
} from '@heroicons/react/24/outline';
import { useMemo } from 'react';
import { ClassDistributionChart } from '../../charts/class-distribution-chart';
import { DataCard, DataItem, SectionHeader } from '../../ui';
import type { CharacterInsightsProps } from './character-insights.types';

export function CharacterInsights({ characters }: CharacterInsightsProps) {
  const metrics = useMemo(() => {
    if (characters.length === 0) {
      return {
        totalCharacters: 0,
        highestLevel: 0,
        averageLevel: 0,
        totalPlayTime: 0,
        totalDeaths: 0,
        hardcoreCount: 0,
        ssfCount: 0,
        classDistribution: {},
        leagueDistribution: {},
        mostPlayedCharacter: null,
      };
    }

    const totalCharacters = characters.length;
    const levels = characters.map(c => c.level);
    const highestLevel = Math.max(...levels);
    const averageLevel = Math.round(
      levels.reduce((sum, level) => sum + level, 0) / totalCharacters
    );

    const totalPlayTime = characters.reduce(
      (sum, c) => sum + c.summary.total_play_time,
      0
    );
    const totalDeaths = characters.reduce(
      (sum, c) => sum + c.summary.total_deaths,
      0
    );

    const hardcoreCount = characters.filter(c => c.hardcore).length;
    const ssfCount = characters.filter(c => c.solo_self_found).length;

    const classDistribution = characters.reduce(
      (acc, c) => {
        acc[c.class] = (acc[c.class] || 0) + 1;
        return acc;
      },
      {} as Record<string, number>
    );

    const leagueDistribution = characters.reduce(
      (acc, c) => {
        acc[c.league] = (acc[c.league] || 0) + 1;
        return acc;
      },
      {} as Record<string, number>
    );

    const mostPlayedCharacter = characters.reduce((max, c) =>
      c.summary.total_play_time > max.summary.total_play_time ? c : max
    );

    return {
      totalCharacters,
      highestLevel,
      averageLevel,
      totalPlayTime,
      totalDeaths,
      hardcoreCount,
      ssfCount,
      classDistribution,
      leagueDistribution,
      mostPlayedCharacter,
    };
  }, [characters]);

  const formatPlayTime = (seconds: number) => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);

    if (hours > 0) {
      return `${hours}h ${minutes}m`;
    }
    return `${minutes}m`;
  };

  if (characters.length === 0) {
    return (
      <DataCard
        title='Character Insights'
        icon={<ChartBarIcon className='w-5 h-5' />}
        isEmpty={true}
        emptyTitle='No Characters to Analyze'
        emptyDescription='Create some characters to view insights'
        emptyIcon={<InboxIcon className='w-8 h-8' />}
      >
        <div></div>
      </DataCard>
    );
  }

  return (
    <DataCard title='Insights' icon={<ChartBarIcon className='w-5 h-5' />}>
      {/* Overview Section */}
      <SectionHeader
        title='Overview'
        icon={<UserGroupIcon className='w-4 h-4' />}
      />
      <div className='space-y-2'>
        {metrics.mostPlayedCharacter && (
          <DataItem
            label='Most Played'
            value={metrics.mostPlayedCharacter.name}
            subValue={`Level ${metrics.mostPlayedCharacter.level} • ${formatPlayTime(
              metrics.mostPlayedCharacter.summary.total_play_time
            )}`}
          />
        )}
        <DataItem label='Total Characters' value={metrics.totalCharacters} />
        <DataItem label='Highest Level' value={metrics.highestLevel} />
        <DataItem label='Average Level' value={metrics.averageLevel} />
      </div>

      {/* Playtime Section */}
      <SectionHeader
        title='Playtime'
        icon={<ChartBarIcon className='w-4 h-4' />}
      />
      <div className='space-y-2'>
        <DataItem
          label='Total Play Time'
          value={formatPlayTime(metrics.totalPlayTime)}
        />
        <DataItem label='Total Deaths' value={metrics.totalDeaths} />
      </div>

      {/* Game Modes Section */}
      <SectionHeader
        title='Game Modes'
        icon={<BoltIcon className='w-4 h-4' />}
      />
      <div className='space-y-2'>
        <DataItem label='Hardcore Characters' value={metrics.hardcoreCount} />
        <DataItem label='SSF Characters' value={metrics.ssfCount} />
      </div>

      {/* Class Distribution Chart */}
      {Object.keys(metrics.classDistribution).length > 0 && (
        <ClassDistributionChart classDistribution={metrics.classDistribution} />
      )}

      {/* League Distribution */}
      {Object.keys(metrics.leagueDistribution).length > 0 && (
        <>
          <SectionHeader
            title='Leagues'
            icon={<GlobeAltIcon className='w-4 h-4' />}
          />
          <div className='space-y-2'>
            {Object.entries(metrics.leagueDistribution)
              .sort(([, a], [, b]) => b - a)
              .map(([league, count]) => (
                <DataItem key={league} label={league} value={count} />
              ))}
          </div>
        </>
      )}
    </DataCard>
  );
}
