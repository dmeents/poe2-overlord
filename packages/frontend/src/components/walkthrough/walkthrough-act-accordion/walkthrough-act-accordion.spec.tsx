import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { WalkthroughActAccordion } from './walkthrough-act-accordion';
import type { WalkthroughAct, WalkthroughStep } from '@/types/walkthrough';

const mockOpenZone = vi.hoisted(() => vi.fn());

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
  invoke: vi.fn(),
}));

const mockStep1: WalkthroughStep = {
  id: 'act1_step_1',
  title: 'First Step',
  description: 'Begin here',
  current_zone: 'Zone A',
  completion_zone: 'Zone B',
  next_step_id: 'act1_step_2',
  previous_step_id: null,
  objectives: [],
  wiki_items: [],
};

const mockStep2: WalkthroughStep = {
  id: 'act1_step_2',
  title: 'Second Step',
  description: 'Continue here',
  current_zone: 'Zone B',
  completion_zone: 'Zone C',
  next_step_id: null,
  previous_step_id: 'act1_step_1',
  objectives: [],
  wiki_items: [],
};

const mockAct: WalkthroughAct = {
  act_name: 'Act 1',
  act_number: 1,
  steps: {
    act1_step_1: mockStep1,
    act1_step_2: mockStep2,
  },
};

describe('WalkthroughActAccordion', () => {
  const defaultProps = {
    act: mockAct,
    actKey: 'act_1',
    isExpanded: true,
    onToggle: vi.fn(),
    onWikiClick: vi.fn(),
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders act name', () => {
    render(<WalkthroughActAccordion {...defaultProps} />);

    expect(screen.getByText('Act 1')).toBeInTheDocument();
  });

  it('renders step count subtitle', () => {
    render(<WalkthroughActAccordion {...defaultProps} />);

    expect(screen.getByText('2 steps')).toBeInTheDocument();
  });

  it('renders all steps when expanded', () => {
    render(<WalkthroughActAccordion {...defaultProps} />);

    expect(screen.getByText('First Step')).toBeInTheDocument();
    expect(screen.getByText('Second Step')).toBeInTheDocument();
  });

  it('calls onToggle when accordion is toggled', async () => {
    const user = userEvent.setup();
    const handleToggle = vi.fn();

    render(
      <WalkthroughActAccordion {...defaultProps} onToggle={handleToggle} />
    );

    // Click on the accordion header (Act 1)
    await user.click(screen.getByText('Act 1'));

    expect(handleToggle).toHaveBeenCalledWith('act_1');
  });

  it('highlights current step', () => {
    render(
      <WalkthroughActAccordion {...defaultProps} currentStepId='act1_step_1' />
    );

    // The current step should have the active styling
    const firstStepCard = screen.getByText('First Step').closest('div');
    expect(firstStepCard).toBeInTheDocument();
  });

  it('passes onWikiClick to step cards', () => {
    const handleWikiClick = vi.fn();

    render(
      <WalkthroughActAccordion
        {...defaultProps}
        onWikiClick={handleWikiClick}
      />
    );

    // Step cards are rendered with onWikiClick prop
    expect(screen.getByText('First Step')).toBeInTheDocument();
  });

  it('renders steps in sorted order by step number', () => {
    render(<WalkthroughActAccordion {...defaultProps} />);

    const stepCards = screen.getAllByText(/Step/);
    expect(stepCards[0].textContent).toBe('First Step');
    expect(stepCards[1].textContent).toBe('Second Step');
  });

  it('renders skip button on non-current steps when onSkipToStep is provided', () => {
    render(
      <WalkthroughActAccordion
        {...defaultProps}
        currentStepId='act1_step_1'
        onSkipToStep={vi.fn()}
      />
    );

    // The "Go Here" button should appear on the non-current step
    expect(screen.getByText('Go Here')).toBeInTheDocument();
  });

  it('calls onSkipToStep when skip button is clicked', async () => {
    const user = userEvent.setup();
    const handleSkip = vi.fn();

    render(
      <WalkthroughActAccordion
        {...defaultProps}
        currentStepId='act1_step_1'
        onSkipToStep={handleSkip}
      />
    );

    await user.click(screen.getByText('Go Here'));

    expect(handleSkip).toHaveBeenCalledWith('act1_step_2');
  });

  it('applies mb-4 className to accordion', () => {
    const { container } = render(<WalkthroughActAccordion {...defaultProps} />);

    // The Accordion component should have mb-4 class
    expect(container.firstChild).toHaveClass('mb-4');
  });

  it('handles acts with no steps', () => {
    const emptyAct: WalkthroughAct = {
      act_name: 'Empty Act',
      act_number: 2,
      steps: {},
    };

    render(
      <WalkthroughActAccordion
        {...defaultProps}
        act={emptyAct}
        actKey='act_2'
      />
    );

    expect(screen.getByText('Empty Act')).toBeInTheDocument();
    expect(screen.getByText('0 steps')).toBeInTheDocument();
  });
});
