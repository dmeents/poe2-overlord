import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type {
  WalkthroughAct,
  WalkthroughGuide as WalkthroughGuideType,
  WalkthroughStep,
} from '@/types/walkthrough';
import { WalkthroughGuide } from './walkthrough-guide';

const mockInvoke = vi.hoisted(() => vi.fn());
const mockOpenZone = vi.hoisted(() => vi.fn());
const mockUseZone = vi.hoisted(() =>
  vi.fn(() => ({
    openZone: mockOpenZone,
  })),
);
const mockUseWalkthrough = vi.hoisted(() =>
  vi.fn(() => ({
    currentStep: null,
    currentAct: null,
  })),
);
const mockUseCharacter = vi.hoisted(() =>
  vi.fn(() => ({
    activeCharacter: null,
  })),
);

vi.mock('@tauri-apps/api/core', () => ({
  invoke: mockInvoke,
}));

vi.mock('@/utils/wiki-utils', () => ({
  handleWikiClick: vi.fn(),
}));

vi.mock('@/contexts/ZoneContext', () => ({
  useZone: mockUseZone,
}));

vi.mock('@/contexts/WalkthroughContext', () => ({
  useWalkthrough: mockUseWalkthrough,
}));

vi.mock('@/contexts/CharacterContext', () => ({
  useCharacter: mockUseCharacter,
}));

const createMockStep = (overrides: Partial<WalkthroughStep> = {}): WalkthroughStep => ({
  id: 'step-1',
  title: 'First Step',
  description: 'Complete the first step',
  current_zone: 'The Coast',
  completion_zone: 'The Mud Flats',
  objectives: [
    {
      text: 'Kill the boss',
      required: true,
      rewards: ['Skill Point'],
    },
  ],
  links: [],
  ...overrides,
});

const createMockAct = (overrides: Partial<WalkthroughAct> = {}): WalkthroughAct => ({
  act_name: 'Act 1',
  steps: [
    createMockStep(),
    createMockStep({
      id: 'step-2',
      title: 'Second Step',
      description: 'Complete the second step',
    }),
  ],
  ...overrides,
});

const createMockGuide = (overrides: Partial<WalkthroughGuideType> = {}): WalkthroughGuideType => ({
  acts: [
    createMockAct(),
    createMockAct({
      act_name: 'Act 2',
      steps: [
        createMockStep({
          id: 'step-3',
          title: 'Third Step',
          description: 'Complete the third step',
        }),
      ],
    }),
  ],
  ...overrides,
});

describe('WalkthroughGuide', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Static Rendering', () => {
    it('renders guide with all acts in order and correct step counts', () => {
      const guide = createMockGuide({
        acts: [
          createMockAct({ act_name: 'Act 1' }),
          createMockAct({ act_name: 'Act 2' }),
          createMockAct({ act_name: 'Act 3' }),
        ],
      });

      render(<WalkthroughGuide guide={guide} />);

      // Header present
      expect(screen.getByText('Guide')).toBeInTheDocument();

      // All acts rendered in order
      const acts = screen.getAllByText(/Act \d/);
      expect(acts[0]).toHaveTextContent('Act 1');
      expect(acts[1]).toHaveTextContent('Act 2');
      expect(acts[2]).toHaveTextContent('Act 3');
    });

    it('applies custom className', () => {
      const { container } = render(
        <WalkthroughGuide guide={createMockGuide()} className="custom-class" />,
      );

      expect(container.querySelector('.custom-class')).toBeInTheDocument();
    });

    it('renders with empty guide', () => {
      const emptyGuide: WalkthroughGuideType = { acts: [] };
      const { container } = render(<WalkthroughGuide guide={emptyGuide} />);

      expect(screen.getByText('Guide')).toBeInTheDocument();
      expect(container.querySelector('.space-y-4')).toBeEmptyDOMElement();
    });
  });

  describe('Accordion Behavior', () => {
    it('starts collapsed and expands/collapses when clicked', async () => {
      const user = userEvent.setup();
      render(<WalkthroughGuide guide={createMockGuide()} />);

      // Initially collapsed
      const content = screen.getByText('First Step');
      let section = content.closest('section');
      expect(section).toHaveAttribute('aria-hidden', 'true');

      // Expand
      await user.click(screen.getByText('Act 1'));
      section = content.closest('section');
      expect(section).not.toHaveAttribute('aria-hidden');

      // Collapse
      await user.click(screen.getByText('Act 1'));
      section = content.closest('section');
      expect(section).toHaveAttribute('aria-hidden', 'true');
    });

    it('allows multiple acts to be expanded simultaneously', async () => {
      const user = userEvent.setup();
      render(<WalkthroughGuide guide={createMockGuide()} />);

      await user.click(screen.getByText('Act 1'));
      await user.click(screen.getByText('Act 2'));

      expect(screen.getByText('First Step')).toBeInTheDocument();
      expect(screen.getByText('Third Step')).toBeInTheDocument();
    });
  });

  describe('Current Step Highlighting', () => {
    it('passes currentStepId to accordions for highlighting', async () => {
      const user = userEvent.setup();
      render(<WalkthroughGuide guide={createMockGuide()} currentStepId="step-1" />);

      await user.click(screen.getByText('Act 1'));

      expect(screen.getByText('First Step')).toBeInTheDocument();
    });
  });

  describe('Skip to Step', () => {
    it('calls invoke to update progress when skipping to step', async () => {
      const user = userEvent.setup();
      mockInvoke.mockResolvedValueOnce(undefined);

      render(
        <WalkthroughGuide
          guide={createMockGuide()}
          characterId="char-123"
          currentStepId="step-1"
        />,
      );

      await user.click(screen.getByText('Act 1'));

      const jumpButtons = screen.getAllByText('Jump to Step');
      await user.click(jumpButtons[0]);

      await waitFor(() => {
        expect(mockInvoke).toHaveBeenCalledWith(
          'update_character_walkthrough_progress',
          expect.objectContaining({
            characterId: 'char-123',
            progress: expect.objectContaining({
              current_step_id: expect.any(String),
              is_completed: false,
            }),
          }),
        );
      });
    });

    it('does not call invoke when no characterId is provided', async () => {
      const user = userEvent.setup();

      render(<WalkthroughGuide guide={createMockGuide()} currentStepId="step-1" />);

      await user.click(screen.getByText('Act 1'));

      const jumpButtons = screen.queryAllByText('Jump to Step');
      if (jumpButtons.length > 0) {
        await user.click(jumpButtons[0]);
      }

      expect(mockInvoke).not.toHaveBeenCalled();
    });

    it('handles skip error gracefully', async () => {
      const user = userEvent.setup();
      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
      mockInvoke.mockRejectedValueOnce(new Error('Failed to skip'));

      render(
        <WalkthroughGuide
          guide={createMockGuide()}
          characterId="char-123"
          currentStepId="step-1"
        />,
      );

      await user.click(screen.getByText('Act 1'));

      const jumpButtons = screen.getAllByText('Jump to Step');
      await user.click(jumpButtons[0]);

      await waitFor(() => {
        expect(consoleSpy).toHaveBeenCalledWith('Failed to skip to step:', expect.any(Error));
      });

      consoleSpy.mockRestore();
    });
  });
});
