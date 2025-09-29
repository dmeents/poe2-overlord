import {
  BoltIcon,
  ChartBarIcon,
  GlobeAltIcon,
  InboxIcon,
  UsersIcon,
} from '@heroicons/react/24/outline';
import { useMemo } from 'react';
import {
  getDistributionClasses,
  getDistributionItemClasses,
  getDistributionLabelClasses,
  getDistributionValueClasses,
  getMetricItemClasses,
  getMetricLabelClasses,
  getMetricValueClasses,
  getMetricsCardClasses,
  getMetricsGridClasses,
} from './character-insights.styles';
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
      <div className={getMetricsCardClasses()}>
        <h3 className='text-lg font-semibold text-white mb-4 flex items-center'>
          <ChartBarIcon className='w-5 h-5 mr-2 text-zinc-400' />
          Character Insights
        </h3>
        <div className='text-center py-8'>
          <div className='w-16 h-16 bg-zinc-800/50 flex items-center justify-center mx-auto mb-4'>
            <InboxIcon className='w-8 h-8 text-zinc-500' />
          </div>
          <p className='text-zinc-400'>No characters to analyze</p>
        </div>
      </div>
    );
  }

  return (
    <div className={getMetricsCardClasses()}>
      <h3 className='text-lg font-semibold text-white mb-6 flex items-center'>
        <ChartBarIcon className='w-5 h-5 mr-2 text-zinc-400' />
        Insights
      </h3>

      {/* Most Played Character */}
      {metrics.mostPlayedCharacter && (
        <div className='mt-6 mb-4 p-4 bg-zinc-900/80 border border-zinc-700/50'>
          <h4 className='text-sm font-medium text-zinc-300 mb-2 flex items-center'>
            <BoltIcon className='w-4 h-4 mr-2 text-zinc-400' />
            Most Played
          </h4>
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
      <div className={getMetricsGridClasses()}>
        <div className={getMetricItemClasses()}>
          <div className={getMetricValueClasses()}>
            {metrics.totalCharacters}
          </div>
          <div className={getMetricLabelClasses()}>Characters</div>
        </div>

        <div className={getMetricItemClasses()}>
          <div className={getMetricValueClasses()}>{metrics.highestLevel}</div>
          <div className={getMetricLabelClasses()}>Highest Level</div>
        </div>

        <div className={getMetricItemClasses()}>
          <div className={getMetricValueClasses()}>
            {formatPlayTime(metrics.totalPlayTime)}
          </div>
          <div className={getMetricLabelClasses()}>Play Time</div>
        </div>

        <div className={getMetricItemClasses()}>
          <div className={getMetricValueClasses()}>{metrics.totalDeaths}</div>
          <div className={getMetricLabelClasses()}>Deaths</div>
        </div>

        <div className={getMetricItemClasses()}>
          <div className={getMetricValueClasses()}>{metrics.hardcoreCount}</div>
          <div className={getMetricLabelClasses()}>Hardcore</div>
        </div>

        <div className={getMetricItemClasses()}>
          <div className={getMetricValueClasses()}>{metrics.ssfCount}</div>
          <div className={getMetricLabelClasses()}>SSF</div>
        </div>
      </div>

      {/* Class Distribution */}
      {Object.keys(metrics.classDistribution).length > 0 && (
        <div className={getDistributionClasses()}>
          <h4 className='text-sm font-medium text-zinc-300 mb-3 flex items-center'>
            <UsersIcon className='w-4 h-4 mr-2 text-zinc-400' />
            Classes
          </h4>
          <div className='space-y-2'>
            {Object.entries(metrics.classDistribution)
              .sort(([, a], [, b]) => b - a)
              .map(([className, count]) => (
                <div key={className} className={getDistributionItemClasses()}>
                  <span className={getDistributionLabelClasses()}>
                    {className}
                  </span>
                  <span className={getDistributionValueClasses()}>{count}</span>
                </div>
              ))}
          </div>
        </div>
      )}

      {/* League Distribution */}
      {Object.keys(metrics.leagueDistribution).length > 0 && (
        <div className={getDistributionClasses()}>
          <h4 className='text-sm font-medium text-zinc-300 mb-3 flex items-center'>
            <GlobeAltIcon className='w-4 h-4 mr-2 text-zinc-400' />
            Leagues
          </h4>
          <div className='space-y-2'>
            {Object.entries(metrics.leagueDistribution)
              .sort(([, a], [, b]) => b - a)
              .map(([league, count]) => (
                <div key={league} className={getDistributionItemClasses()}>
                  <span className={getDistributionLabelClasses()}>
                    {league}
                  </span>
                  <span className={getDistributionValueClasses()}>{count}</span>
                </div>
              ))}
          </div>
        </div>
      )}
    </div>
  );
}
