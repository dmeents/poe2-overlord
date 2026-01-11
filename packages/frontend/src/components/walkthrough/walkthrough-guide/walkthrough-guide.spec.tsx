import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi, beforeEach } from 'vitest';
import { WalkthroughGuide } from './walkthrough-guide';
import type {
  WalkthroughGuide as WalkthroughGuideType,
  WalkthroughAct,
  WalkthroughStep,
} from '@/types/walkthrough';

const mockInvoke = vi.hoisted(() => vi.fn());
const mockOpenZone = vi.hoisted(() => vi.fn());
const mockUseZone = vi.hoisted(() =>
  vi.fn(() => ({
    openZone: mockOpenZone,
  }))
);
const mockUseWalkthrough = vi.hoisted(() =>
  vi.fn(() => ({
    currentStep: null,
    currentAct: null,
  }))
);
const mockUseCharacter = vi.hoisted(() =>
  vi.fn(() => ({
    activeCharacter: null,
  }))
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

const createMockStep = (
  overrides: Partial<WalkthroughStep> = {}
): WalkthroughStep => ({
  id: 'step-1',
  title: 'First Step',
  description: 'Complete the first step',
  current_zone: 'The Coast',
  completion_zone: 'The Mud Flats',
  next_step_id: 'step-2',
  previous_step_id: null,
  objectives: [
    {
      text: 'Kill the boss',
      required: true,
      rewards: ['Skill Point'],
    },
  ],
  wiki_items: [],
  ...overrides,
});

const createMockAct = (
  overrides: Partial<WalkthroughAct> = {}
): WalkthroughAct => ({
  act_name: 'Act 1',
  act_number: 1,
  steps: {
    'step-1': createMockStep(),
    'step-2': createMockStep({
      id: 'step-2',
      title: 'Second Step',
      description: 'Complete the second step',
      previous_step_id: 'step-1',
      next_step_id: null,
    }),
  },
  ...overrides,
});

const createMockGuide = (
  overrides: Partial<WalkthroughGuideType> = {}
): WalkthroughGuideType => ({
  acts: {
    'act-1': createMockAct(),
    'act-2': createMockAct({
      act_name: 'Act 2',
      act_number: 2,
      steps: {
        'step-3': createMockStep({
          id: 'step-3',
          title: 'Third Step',
          description: 'Complete the third step',
          previous_step_id: 'step-2',
          next_step_id: null,
        }),
      },
    }),
  },
  ...overrides,
});

describe('WalkthroughGuide', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Rendering', () => {
    it('renders the guide section header', () => {
      render(<WalkthroughGuide guide={createMockGuide()} />);

      expect(screen.getByText('Guide')).toBeInTheDocument();
    });

    it('renders all acts from the guide', () => {
      render(<WalkthroughGuide guide={createMockGuide()} />);

      expect(screen.getByText('Act 1')).toBeInTheDocument();
      expect(screen.getByText('Act 2')).toBeInTheDocument();
    });

    it('renders acts in order by act number', () => {
      const guide = createMockGuide({
        acts: {
          'act-3': createMockAct({ act_name: 'Act 3', act_number: 3 }),
          'act-1': createMockAct({ act_name: 'Act 1', act_number: 1 }),
          'act-2': createMockAct({ act_name: 'Act 2', act_number: 2 }),
        },
      });

      render(<WalkthroughGuide guide={guide} />);

      const acts = screen.getAllByText(/Act \d/);
      expect(acts[0]).toHaveTextContent('Act 1');
      expect(acts[1]).toHaveTextContent('Act 2');
      expect(acts[2]).toHaveTextContent('Act 3');
    });

    it('applies custom className', () => {
      const { container } = render(
        <WalkthroughGuide guide={createMockGuide()} className='custom-class' />
      );

      // The className is passed to SectionHeader
      expect(container.querySelector('.custom-class')).toBeInTheDocument();
    });

    it('renders with empty guide', () => {
      const emptyGuide: WalkthroughGuideType = { acts: {} };
      const { container } = render(<WalkthroughGuide guide={emptyGuide} />);

      expect(screen.getByText('Guide')).toBeInTheDocument();
      expect(container.querySelector('.space-y-4')).toBeEmptyDOMElement();
    });
  });

  describe('Accordion Behavior', () => {
    it('starts with all acts collapsed', () => {
      render(<WalkthroughGuide guide={createMockGuide()} />);

      // Steps should not be visible initially
      expect(screen.queryByText('First Step')).not.toBeInTheDocument();
    });

    it('expands an act when clicked', async () => {
      const user = userEvent.setup();
      render(<WalkthroughGuide guide={createMockGuide()} />);

      await user.click(screen.getByText('Act 1'));

      expect(screen.getByText('First Step')).toBeInTheDocument();
    });

    it('collapses an act when clicked again', async () => {
      const user = userEvent.setup();
      render(<WalkthroughGuide guide={createMockGuide()} />);

      // Expand
      await user.click(screen.getByText('Act 1'));
      expect(screen.getByText('First Step')).toBeInTheDocument();

      // Collapse
      await user.click(screen.getByText('Act 1'));
      expect(screen.queryByText('First Step')).not.toBeInTheDocument();
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
    it('passes currentStepId to accordions', async () => {
      const user = userEvent.setup();
      render(
        <WalkthroughGuide guide={createMockGuide()} currentStepId='step-1' />
      );

      await user.click(screen.getByText('Act 1'));

      // The current step should be highlighted (tested in accordion component)
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
          characterId='char-123'
          currentStepId='step-1'
        />
      );

      await user.click(screen.getByText('Act 1'));

      // Find and click the "Go Here" button for non-current steps (title is "Go to this step")
      const goHereButtons = screen.getAllByTitle('Go to this step');
      await user.click(goHereButtons[0]);

      await waitFor(() => {
        expect(mockInvoke).toHaveBeenCalledWith(
          'update_character_walkthrough_progress',
          expect.objectContaining({
            characterId: 'char-123',
            progress: expect.objectContaining({
              current_step_id: expect.any(String),
              is_completed: false,
            }),
          })
        );
      });
    });

    it('does not call invoke when no characterId is provided', async () => {
      const user = userEvent.setup();

      render(
        <WalkthroughGuide guide={createMockGuide()} currentStepId='step-1' />
      );

      await user.click(screen.getByText('Act 1'));

      const goHereButtons = screen.queryAllByTitle('Go to this step');
      if (goHereButtons.length > 0) {
        await user.click(goHereButtons[0]);
      }

      expect(mockInvoke).not.toHaveBeenCalled();
    });

    it('handles skip error gracefully', async () => {
      const user = userEvent.setup();
      const consoleSpy = vi
        .spyOn(console, 'error')
        .mockImplementation(() => {});
      mockInvoke.mockRejectedValueOnce(new Error('Failed to skip'));

      render(
        <WalkthroughGuide
          guide={createMockGuide()}
          characterId='char-123'
          currentStepId='step-1'
        />
      );

      await user.click(screen.getByText('Act 1'));

      const goHereButtons = screen.getAllByTitle('Go to this step');
      await user.click(goHereButtons[0]);

      await waitFor(() => {
        expect(consoleSpy).toHaveBeenCalledWith(
          'Failed to skip to step:',
          expect.any(Error)
        );
      });

      consoleSpy.mockRestore();
    });
  });

  describe('Step Count Display', () => {
    it('shows correct step count for each act', () => {
      render(<WalkthroughGuide guide={createMockGuide()} />);

      // Act 1 has 2 steps, Act 2 has 1 step (component always uses "steps" plural)
      expect(screen.getByText('2 steps')).toBeInTheDocument();
      expect(screen.getByText('1 steps')).toBeInTheDocument();
    });
  });
});
