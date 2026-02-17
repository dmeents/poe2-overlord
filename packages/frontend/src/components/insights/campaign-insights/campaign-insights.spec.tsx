import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';

import type { WalkthroughGuide, WalkthroughStep } from '@/types/walkthrough';
import { CampaignInsights } from './campaign-insights';

const createMockGuide = (numActs = 2, stepsPerAct = 3): WalkthroughGuide => {
  const acts: WalkthroughGuide['acts'] = [];

  for (let i = 1; i <= numActs; i++) {
    const steps: WalkthroughStep[] = [];
    for (let j = 1; j <= stepsPerAct; j++) {
      steps.push({
        id: `step-${i}-${j}`,
        title: `Step ${j}`,
        description: `Description for step ${j}`,
        current_zone: 'Zone A',
        completion_zone: 'Zone B',
        objectives: [],
        links: [],
      });
    }
    acts.push({
      act_name: `Act ${i}`,
      steps,
    });
  }

  return { acts };
};

describe('CampaignInsights', () => {
  describe('Static Rendering', () => {
    it('renders campaign insights information correctly', () => {
      render(<CampaignInsights guide={createMockGuide(2, 3)} />);

      // Card title
      expect(screen.getByText('Insights')).toBeInTheDocument();

      // Total acts count
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
        <CampaignInsights guide={createMockGuide()} className="custom-class" />,
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
        acts: [
          {
            act_name: 'Act 1',
            steps: [{} as never, {} as never],
          },
          {
            act_name: 'Act 2',
            steps: [{} as never, {} as never, {} as never],
          },
        ],
      };

      render(<CampaignInsights guide={guide} />);

      expect(screen.getByText('2')).toBeInTheDocument(); // acts
      expect(screen.getByText('5')).toBeInTheDocument(); // steps
    });

    it('handles empty guide', () => {
      const emptyGuide: WalkthroughGuide = { acts: [] };

      render(<CampaignInsights guide={emptyGuide} />);

      const zeroElements = screen.getAllByText('0');
      expect(zeroElements.length).toBe(2); // Both acts and steps should be 0
    });
  });
});
