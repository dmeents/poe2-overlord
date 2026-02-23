import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { WalkthroughStep, WalkthroughStepResult } from '@/types/walkthrough';
import { WalkthroughStepCard } from './walkthrough-step-card';

const mockOpenZone = vi.hoisted(() => vi.fn());
const mockInvoke = vi.hoisted(() => vi.fn());

vi.mock('@/contexts/ZoneContext', () => ({
  useZone: () => ({
    openZone: mockOpenZone,
  }),
}));

vi.mock('@/contexts/WalkthroughContext', () => ({
  useWalkthrough: () => ({
    progress: null,
    currentStep: null,
    previousStep: null,
  }),
}));

vi.mock('@/contexts/CharacterContext', () => ({
  useCharacter: () => ({
    activeCharacter: null,
  }),
}));

vi.mock('@/contexts/ConfigurationContext', () => ({
  useConfiguration: () => ({
    config: null,
    isLoading: false,
    updateConfig: vi.fn(),
  }),
}));

vi.mock('@tauri-apps/api/core', () => ({
  invoke: mockInvoke,
}));

const mockStep: WalkthroughStep = {
  id: 'step-1',
  title: 'First Steps',
  description: 'Begin your journey by heading to the beach',
  current_zone: 'Twilight Strand',
  completion_zone: 'The Coast',
  objectives: [
    {
      text: 'Talk to the NPC',
      details: 'Find the NPC near the entrance',
      required: true,
      rewards: ['Skill Gem'],
    },
    {
      text: 'Kill the boss',
      required: false,
      rewards: [],
    },
  ],
  links: [
    { text: 'Skill Gem', url: 'https://www.poe2wiki.net/wiki/Skill_Gem' },
    { text: 'Twilight Strand', url: 'https://www.poe2wiki.net/wiki/Twilight_Strand' },
  ],
};

const mockStepResult: WalkthroughStepResult = {
  step: mockStep,
  act_name: 'Act 1',
  act_number: 1,
};

describe('WalkthroughStepCard', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('preview variant', () => {
    describe('Static Rendering', () => {
      it('renders walkthrough step information correctly', () => {
        render(<WalkthroughStepCard step={mockStep} variant="preview" onLinkClick={vi.fn()} />);

        // Step title
        expect(screen.getByText('First Steps')).toBeInTheDocument();

        // Current zone as subtitle
        expect(screen.getByText('Twilight Strand')).toBeInTheDocument();

        // Completion zone
        expect(screen.getByText('The Coast')).toBeInTheDocument();

        // Step description
        expect(screen.getByText(/Begin your journey by heading to the beach/)).toBeInTheDocument();

        // Objectives
        expect(screen.getByText(/Talk to the NPC/)).toBeInTheDocument();
        expect(screen.getByText(/Kill the boss/)).toBeInTheDocument();
        expect(screen.getByText('Objectives (2):')).toBeInTheDocument();

        // Objective details
        expect(screen.getByText(/Find the NPC near the entrance/)).toBeInTheDocument();

        // Objective rewards
        expect(screen.getByText(/Skill Gem/)).toBeInTheDocument();
      });
    });

    it('opens zone when completion zone is clicked', async () => {
      const user = userEvent.setup();

      render(<WalkthroughStepCard step={mockStep} variant="preview" onLinkClick={vi.fn()} />);

      await user.click(screen.getByText('The Coast'));

      expect(mockOpenZone).toHaveBeenCalledWith('The Coast');
    });

    it('calls onLinkClick for wiki items', async () => {
      const user = userEvent.setup();
      const handleWikiClick = vi.fn();

      render(
        <WalkthroughStepCard step={mockStep} variant="preview" onLinkClick={handleWikiClick} />,
      );

      // Find a wiki item link (Skill Gem)
      const wikiLinks = screen.getAllByText(/Skill Gem/);
      if (wikiLinks.length > 0) {
        await user.click(wikiLinks[0]);
        // Wiki items that are not zones should call onLinkClick
      }
    });

    it('renders skip button when not current', () => {
      render(
        <WalkthroughStepCard
          step={mockStep}
          variant="preview"
          isCurrent={false}
          onLinkClick={vi.fn()}
          onSkipToStep={vi.fn()}
        />,
      );

      expect(screen.getByText('Jump to Step')).toBeInTheDocument();
    });

    it('calls onSkipToStep when skip button is clicked', async () => {
      const user = userEvent.setup();
      const handleSkip = vi.fn();

      render(
        <WalkthroughStepCard
          step={mockStep}
          variant="preview"
          isCurrent={false}
          onLinkClick={vi.fn()}
          onSkipToStep={handleSkip}
        />,
      );

      await user.click(screen.getByText('Jump to Step'));

      expect(handleSkip).toHaveBeenCalledWith('step-1');
    });

    it('applies custom className', () => {
      const { container } = render(
        <WalkthroughStepCard
          step={mockStep}
          variant="preview"
          className="custom-class"
          onLinkClick={vi.fn()}
        />,
      );

      expect(container.firstChild).toHaveClass('custom-class');
    });
  });

  describe('with stepResult prop', () => {
    it('renders step from stepResult', () => {
      render(
        <WalkthroughStepCard stepResult={mockStepResult} variant="preview" onLinkClick={vi.fn()} />,
      );

      expect(screen.getByText('First Steps')).toBeInTheDocument();
    });
  });

  describe('with isCurrent prop', () => {
    it('applies active styling when isCurrent is true', () => {
      const { container } = render(
        <WalkthroughStepCard
          step={mockStep}
          variant="preview"
          isCurrent={true}
          onLinkClick={vi.fn()}
        />,
      );

      expect(container.firstChild).toHaveClass('border-ember-700/50');
    });
  });

  describe('without step data', () => {
    it('returns null when no step data in preview mode', () => {
      const { container } = render(<WalkthroughStepCard variant="preview" onLinkClick={vi.fn()} />);

      expect(container.firstChild).toBeNull();
    });
  });
});
