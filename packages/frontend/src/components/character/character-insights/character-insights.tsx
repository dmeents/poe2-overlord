import {
  BoltIcon,
  ChartBarIcon,
  GlobeAltIcon,
  InboxIcon,
} from '@heroicons/react/24/outline';
import { useMemo } from 'react';
import { Card, SectionHeader, StatGrid } from '../';
import { ClassDistributionChart } from '../../charts/class-distribution-chart';
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
      <Card
        title='Character Insights'
        icon={<ChartBarIcon className='w-5 h-5' />}
      >
        <div className='text-center py-8'>
          <div className='w-16 h-16 bg-zinc-800/50 flex items-center justify-center mx-auto mb-4'>
            <InboxIcon className='w-8 h-8 text-zinc-500' />
          </div>
          <p className='text-zinc-400'>No characters to analyze</p>
        </div>
      </Card>
    );
  }

  const stats = [
    { value: metrics.totalCharacters, label: 'Characters' },
    { value: metrics.highestLevel, label: 'Highest Level' },
    { value: formatPlayTime(metrics.totalPlayTime), label: 'Play Time' },
    { value: metrics.totalDeaths, label: 'Deaths' },
    { value: metrics.hardcoreCount, label: 'Hardcore' },
    { value: metrics.ssfCount, label: 'SSF' },
  ];

  return (
    <Card title='Insights' icon={<ChartBarIcon className='w-5 h-5' />}>
      {/* Most Played Character */}
      {metrics.mostPlayedCharacter && (
        <div className='mt-6 mb-4 p-4 bg-zinc-900/80 border border-zinc-700/50'>
          <SectionHeader
            title='Most Played'
            icon={<BoltIcon className='w-4 h-4' />}
          />
          <div className='text-white font-medium'>
            {metrics.mostPlayedCharacter.name}
          </div>
          <div className='text-zinc-400 text-sm'>
            {metrics.mostPlayedCharacter.class} • Level{' '}
            {metrics.mostPlayedCharacter.level} •{' '}
            {formatPlayTime(
              metrics.mostPlayedCharacter.summary.total_play_time
            )}
          </div>
        </div>
      )}

      {/* Main Stats Grid */}
      <StatGrid stats={stats} columns={2} />

      {/* Class Distribution Chart */}
      {Object.keys(metrics.classDistribution).length > 0 && (
        <ClassDistributionChart classDistribution={metrics.classDistribution} />
      )}

      {/* League Distribution */}
      {Object.keys(metrics.leagueDistribution).length > 0 && (
        <div className='mt-6 space-y-4'>
          <SectionHeader
            title='Leagues'
            icon={<GlobeAltIcon className='w-4 h-4' />}
          />
          <div className='space-y-2'>
            {Object.entries(metrics.leagueDistribution)
              .sort(([, a], [, b]) => b - a)
              .map(([league, count]) => (
                <div
                  key={league}
                  className='flex items-center justify-between p-3 bg-zinc-900/80 border border-zinc-700/50'
                >
                  <span className='text-zinc-300 font-medium'>{league}</span>
                  <span className='text-zinc-400 text-sm'>{count}</span>
                </div>
              ))}
          </div>
        </div>
      )}
    </Card>
  );
}
