import { Card } from '@/components/ui/card/card';
import { Toggle } from '@/components/ui/toggle/toggle';
import { useConfiguration } from '@/contexts/ConfigurationContext';
import { walkthroughSettingsPanelStyles as styles } from './walkthrough-settings-panel.styles';

interface WalkthroughSettingsPanelProps {
  variant: 'card' | 'inline';
}

const SETTINGS = [
  {
    field: 'hide_optional_objectives' as const,
    label: 'Hide optional objectives',
    shortLabel: 'Optional',
    description: 'Hides objectives that are not required for campaign progression.',
  },
  {
    field: 'hide_league_start_objectives' as const,
    label: 'Hide league start objectives',
    shortLabel: 'League Start',
    description: 'Hides objectives only relevant for your first character per league.',
  },
  {
    field: 'hide_flavor_text' as const,
    label: 'Hide flavor text',
    shortLabel: 'Flavor Text',
    description: 'Hides narrative descriptions on each step.',
  },
  {
    field: 'hide_objective_descriptions' as const,
    label: 'Hide objective descriptions',
    shortLabel: 'Descriptions',
    description: 'Hides detail text below objective headings.',
  },
] as const;

export function WalkthroughSettingsPanel({ variant }: WalkthroughSettingsPanelProps) {
  const { config, updateConfig } = useConfiguration();

  if (variant === 'inline') {
    return (
      <div className={styles.inline.container}>
        <span className={styles.inline.label}>Hide:</span>
        {SETTINGS.map(({ field, shortLabel }) => (
          <span key={field} className={styles.inline.item}>
            <Toggle
              checked={config?.[field] ?? false}
              onChange={checked => updateConfig({ [field]: checked })}
              size="sm"
            />
            <span className={styles.inline.itemLabel}>{shortLabel}</span>
          </span>
        ))}
      </div>
    );
  }

  return (
    <Card title="Walkthrough Display">
      <div className="px-4 py-2">
        {SETTINGS.map(({ field, label, description }) => (
          <div key={field} className={styles.card.row}>
            <div className={styles.card.label}>
              <p className={styles.card.labelText}>{label}</p>
              <p className={styles.card.descriptionText}>{description}</p>
            </div>
            <Toggle
              checked={config?.[field] ?? false}
              onChange={checked => updateConfig({ [field]: checked })}
              size="md"
            />
          </div>
        ))}
      </div>
    </Card>
  );
}
