import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { CampaignInsights } from './campaign-insights';
import type { WalkthroughGuide } from '@/types/walkthrough';

const createMockGuide = (numActs = 2, stepsPerAct = 3): WalkthroughGuide => {
  const acts: WalkthroughGuide['acts'] = {};

  for (let i = 1; i <= numActs; i++) {
    const steps: Record<string, unknown> = {};
    for (let j = 1; j <= stepsPerAct; j++) {
      steps[`step-${i}-${j}`] = {
        id: `step-${i}-${j}`,
        title: `Step ${j}`,
        description: `Description for step ${j}`,
        current_zone: 'Zone A',
        completion_zone: 'Zone B',
        next_step_id: null,
        previous_step_id: null,
        objectives: [],
        wiki_items: [],
      };
    }
    acts[`act-${i}`] = {
      act_name: `Act ${i}`,
      act_number: i,
      steps,
    };
  }

  return { acts } as unknown as WalkthroughGuide;
};

describe('CampaignInsights', () => {
  describe('Rendering', () => {
    it('renders card with Insights title', () => {
      render(<CampaignInsights guide={createMockGuide()} />);

      expect(screen.getByText('Insights')).toBeInTheDocument();
    });

    it('renders total acts count', () => {
      render(<CampaignInsights guide={createMockGuide(2, 3)} />);

      expect(screen.getByText('Total Acts')).toBeInTheDocument();
      expect(screen.getByText('2')).toBeInTheDocument();
    });

    it('renders total steps count', () => {
      render(<CampaignInsights guide={createMockGuide(2, 3)} />);

      expect(screen.getByText('Total Steps')).toBeInTheDocument();
      // 2 acts * 3 steps = 6 total steps
      expect(screen.getByText('6')).toBeInTheDocument();
    });

    it('applies custom className', () => {
      const { container } = render(
        <CampaignInsights guide={createMockGuide()} className="custom-class" />
      );

      expect(container.querySelector('.custom-class')).toBeInTheDocument();
    });
  });

  describe('Calculations', () => {
    it('calculates correct total for single act', () => {
      render(<CampaignInsights guide={createMockGuide(1, 5)} />);

      expect(screen.getByText('1')).toBeInTheDocument(); // acts
      expect(screen.getByText('5')).toBeInTheDocument(); // steps
    });

    it('calculates correct total for multiple acts with varying steps', () => {
      const guide: WalkthroughGuide = {
        acts: {
          'act-1': {
            act_name: 'Act 1',
            act_number: 1,
            steps: {
              'step-1': {} as never,
              'step-2': {} as never,
            },
          },
          'act-2': {
            act_name: 'Act 2',
            act_number: 2,
            steps: {
              'step-3': {} as never,
              'step-4': {} as never,
              'step-5': {} as never,
            },
          },
        },
      };

      render(<CampaignInsights guide={guide} />);

      expect(screen.getByText('2')).toBeInTheDocument(); // acts
      expect(screen.getByText('5')).toBeInTheDocument(); // steps
    });

    it('handles empty guide', () => {
      const emptyGuide: WalkthroughGuide = { acts: {} };

      render(<CampaignInsights guide={emptyGuide} />);

      const zeroElements = screen.getAllByText('0');
      expect(zeroElements.length).toBe(2); // Both acts and steps should be 0
    });
  });
});
