import { BookOpenIcon, ClockIcon } from '@heroicons/react/24/outline';
import type { CharacterData } from '@/types/character';
import { Button } from '../../ui/button/button';
import { Card } from '../../ui/card/card';
import { campaignCompleteCardStyles as styles } from './campaign-complete-card.styles';

interface CampaignCompleteCardProps {
  lastUpdated: string;
  character?: CharacterData;
  onViewGuide?: () => void;
  className?: string;
}

export function CampaignCompleteCard({
  lastUpdated,
  character,
  onViewGuide,
  className = '',
}: CampaignCompleteCardProps): React.JSX.Element {
  const formatCompletionText = () => {
    const date = new Date(lastUpdated);
    const dateString = date.toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    });

    // Calculate total campaign time if character data is available
    if (character) {
      const totalTimeSeconds =
        character.summary.play_time_act1 +
        character.summary.play_time_act2 +
        character.summary.play_time_act3 +
        character.summary.play_time_act4 +
        character.summary.play_time_act5 +
        character.summary.play_time_interlude;

      const hours = Math.round(totalTimeSeconds / 3600);
      const hoursText = hours === 1 ? 'hour' : 'hours';
      return `Completed in ${hours} ${hoursText} on ${dateString}`;
    }

    return `Completed on ${dateString}`;
  };

  return (
    <Card className={`${className} ${styles.container} ${styles.background}`}>
      {/* Decorative top accent */}
      <div className={styles.borderAccent} />

      {/* Main content */}
      <div className={styles.content}>
        {/* Title */}
        <h3 className={`${styles.title} ${styles.titleGlow}`}>Campaign Conquered</h3>

        {/* Message */}
        <p className={styles.message}>
          Your journey through Wraeclast is complete. The path ahead lies in the endgame, where
          greater challenges and rewards await.
        </p>

        {/* Timestamp */}
        <div className={styles.timestamp}>
          <ClockIcon className={styles.timestampIcon} />
          {formatCompletionText()}
        </div>
      </div>

      {/* Footer */}
      {onViewGuide && (
        <div className={styles.footer}>
          <Button onClick={onViewGuide} variant="text" size="xs" title="View guide">
            <BookOpenIcon className="w-3 h-3" />
            Guide
          </Button>
        </div>
      )}
    </Card>
  );
}
