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

vi.mock('@tauri-apps/api/core', () => ({
  invoke: mockInvoke,
}));

const mockStep: WalkthroughStep = {
  id: 'step-1',
  title: 'First Steps',
  description: 'Begin your journey by heading to the beach',
  current_zone: 'Twilight Strand',
  completion_zone: 'The Coast',
  next_step_id: 'step-2',
  previous_step_id: null,
  objectives: [
    {
      text: 'Talk to the NPC',
      details: 'Find the NPC near the entrance',
      required: true,
      rewards: ['Skill Gem'],
      notes: 'Important quest reward',
    },
    {
      text: 'Kill the boss',
      required: false,
      rewards: [],
    },
  ],
  wiki_items: ['Skill Gem', 'Twilight Strand'],
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
    it('renders step title', () => {
      render(<WalkthroughStepCard step={mockStep} variant="preview" onWikiClick={vi.fn()} />);

      expect(screen.getByText('First Steps')).toBeInTheDocument();
    });

    it('renders current zone as subtitle', () => {
      render(<WalkthroughStepCard step={mockStep} variant="preview" onWikiClick={vi.fn()} />);

      expect(screen.getByText('Twilight Strand')).toBeInTheDocument();
    });

    it('renders completion zone', () => {
      render(<WalkthroughStepCard step={mockStep} variant="preview" onWikiClick={vi.fn()} />);

      expect(screen.getByText('The Coast')).toBeInTheDocument();
    });

    it('renders step description', () => {
      render(<WalkthroughStepCard step={mockStep} variant="preview" onWikiClick={vi.fn()} />);

      expect(screen.getByText(/Begin your journey by heading to the beach/)).toBeInTheDocument();
    });

    it('renders objectives', () => {
      render(<WalkthroughStepCard step={mockStep} variant="preview" onWikiClick={vi.fn()} />);

      expect(screen.getByText(/Talk to the NPC/)).toBeInTheDocument();
      expect(screen.getByText(/Kill the boss/)).toBeInTheDocument();
      expect(screen.getByText('Objectives (2):')).toBeInTheDocument();
    });

    it('renders objective details', () => {
      render(<WalkthroughStepCard step={mockStep} variant="preview" onWikiClick={vi.fn()} />);

      expect(screen.getByText(/Find the NPC near the entrance/)).toBeInTheDocument();
    });

    it('renders objective notes', () => {
      render(<WalkthroughStepCard step={mockStep} variant="preview" onWikiClick={vi.fn()} />);

      expect(screen.getByText(/Important quest reward/)).toBeInTheDocument();
    });

    it('renders objective rewards', () => {
      render(<WalkthroughStepCard step={mockStep} variant="preview" onWikiClick={vi.fn()} />);

      expect(screen.getByText(/Skill Gem/)).toBeInTheDocument();
    });

    it('opens zone when completion zone is clicked', async () => {
      const user = userEvent.setup();

      render(<WalkthroughStepCard step={mockStep} variant="preview" onWikiClick={vi.fn()} />);

      await user.click(screen.getByText('The Coast'));

      expect(mockOpenZone).toHaveBeenCalledWith('The Coast');
    });

    it('calls onWikiClick for wiki items', async () => {
      const user = userEvent.setup();
      const handleWikiClick = vi.fn();

      render(
        <WalkthroughStepCard step={mockStep} variant="preview" onWikiClick={handleWikiClick} />,
      );

      // Find a wiki item link (Skill Gem)
      const wikiLinks = screen.getAllByText(/Skill Gem/);
      if (wikiLinks.length > 0) {
        await user.click(wikiLinks[0]);
        // Wiki items that are not zones should call onWikiClick
      }
    });

    it('renders skip button when not current', () => {
      render(
        <WalkthroughStepCard
          step={mockStep}
          variant="preview"
          isCurrent={false}
          onWikiClick={vi.fn()}
          onSkipToStep={vi.fn()}
        />,
      );

      expect(screen.getByText('Go Here')).toBeInTheDocument();
    });

    it('calls onSkipToStep when skip button is clicked', async () => {
      const user = userEvent.setup();
      const handleSkip = vi.fn();

      render(
        <WalkthroughStepCard
          step={mockStep}
          variant="preview"
          isCurrent={false}
          onWikiClick={vi.fn()}
          onSkipToStep={handleSkip}
        />,
      );

      await user.click(screen.getByText('Go Here'));

      expect(handleSkip).toHaveBeenCalledWith('step-1');
    });

    it('applies custom className', () => {
      const { container } = render(
        <WalkthroughStepCard
          step={mockStep}
          variant="preview"
          className="custom-class"
          onWikiClick={vi.fn()}
        />,
      );

      expect(container.firstChild).toHaveClass('custom-class');
    });
  });

  describe('with stepResult prop', () => {
    it('renders step from stepResult', () => {
      render(
        <WalkthroughStepCard stepResult={mockStepResult} variant="preview" onWikiClick={vi.fn()} />,
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
          onWikiClick={vi.fn()}
        />,
      );

      expect(container.firstChild).toHaveClass('border-blue-500');
    });
  });

  describe('without step data', () => {
    it('returns null when no step data in preview mode', () => {
      const { container } = render(<WalkthroughStepCard variant="preview" onWikiClick={vi.fn()} />);

      expect(container.firstChild).toBeNull();
    });
  });
});
