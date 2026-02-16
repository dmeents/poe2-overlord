import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { createMockObjective } from '../../../test/mock-data';
import { StepObjectiveList } from './step-objective-list';

describe('StepObjectiveList', () => {
  const defaultProps = {
    objectives: [createMockObjective({ text: 'Test objective' })],
    wikiItems: [],
    onWikiClick: vi.fn(),
  };

  it('returns null when objectives array is empty', () => {
    const { container } = render(<StepObjectiveList {...defaultProps} objectives={[]} />);

    expect(container.firstChild).toBeNull();
  });

  it('renders objectives heading with count', () => {
    render(<StepObjectiveList {...defaultProps} />);

    expect(screen.getByText('Objectives (1):')).toBeInTheDocument();
  });

  it('renders objective text', () => {
    render(<StepObjectiveList {...defaultProps} />);

    expect(screen.getByText('Test objective')).toBeInTheDocument();
  });

  it('renders multiple objectives', () => {
    const objectives = [
      createMockObjective({ text: 'First objective' }),
      createMockObjective({ text: 'Second objective' }),
      createMockObjective({ text: 'Third objective' }),
    ];

    render(<StepObjectiveList {...defaultProps} objectives={objectives} />);

    expect(screen.getByText('Objectives (3):')).toBeInTheDocument();
    expect(screen.getByText('First objective')).toBeInTheDocument();
    expect(screen.getByText('Second objective')).toBeInTheDocument();
    expect(screen.getByText('Third objective')).toBeInTheDocument();
  });

  it('renders required objective icon', () => {
    const objectives = [createMockObjective({ text: 'Required objective', required: true })];

    render(<StepObjectiveList {...defaultProps} objectives={objectives} />);

    // Icon with title "Required" should be present
    expect(screen.getByTitle('Required')).toBeInTheDocument();
  });

  it('renders optional objective icon', () => {
    const objectives = [createMockObjective({ text: 'Optional objective', required: false })];

    render(<StepObjectiveList {...defaultProps} objectives={objectives} />);

    // Icon with title "Optional" should be present
    expect(screen.getByTitle('Optional')).toBeInTheDocument();
  });

  it('renders objective details', () => {
    const objectives = [
      createMockObjective({
        text: 'Main objective',
        details: 'Additional details here',
      }),
    ];

    render(<StepObjectiveList {...defaultProps} objectives={objectives} />);

    expect(screen.getByText('Additional details here')).toBeInTheDocument();
  });

  it('renders objective notes', () => {
    const objectives = [
      createMockObjective({
        text: 'Main objective',
        notes: 'Important note',
      }),
    ];

    render(<StepObjectiveList {...defaultProps} objectives={objectives} />);

    expect(screen.getByText(/Note:/)).toBeInTheDocument();
    expect(screen.getByText(/Important note/)).toBeInTheDocument();
  });

  it('renders objective rewards', () => {
    const objectives = [
      createMockObjective({
        text: 'Main objective',
        rewards: ['Skill Point', 'Quest Item'],
      }),
    ];

    render(<StepObjectiveList {...defaultProps} objectives={objectives} />);

    expect(screen.getByText(/Skill Point, Quest Item/)).toBeInTheDocument();
  });

  it('renders rewards icon', () => {
    const objectives = [
      createMockObjective({
        text: 'Main objective',
        rewards: ['Reward'],
      }),
    ];

    render(<StepObjectiveList {...defaultProps} objectives={objectives} />);

    const icon = screen.getByTitle('Rewards');
    expect(icon).toBeInTheDocument();
  });

  it('renders wiki links in objective text', () => {
    const objectives = [createMockObjective({ text: 'Defeat Hillock' })];

    render(<StepObjectiveList {...defaultProps} objectives={objectives} wikiItems={['Hillock']} />);

    expect(screen.getByRole('button', { name: 'Hillock' })).toBeInTheDocument();
  });

  it('calls onWikiClick when wiki link is clicked', async () => {
    const user = userEvent.setup();
    const handleWikiClick = vi.fn();
    const objectives = [createMockObjective({ text: 'Defeat Hillock' })];

    render(
      <StepObjectiveList
        {...defaultProps}
        objectives={objectives}
        wikiItems={['Hillock']}
        onWikiClick={handleWikiClick}
      />,
    );

    await user.click(screen.getByRole('button', { name: 'Hillock' }));

    expect(handleWikiClick).toHaveBeenCalledWith('Hillock');
  });

  it('renders wiki links in details', () => {
    const objectives = [
      createMockObjective({
        text: 'Main objective',
        details: 'Find the Karui Fortress',
      }),
    ];

    render(
      <StepObjectiveList
        {...defaultProps}
        objectives={objectives}
        wikiItems={['Karui Fortress']}
      />,
    );

    expect(screen.getByRole('button', { name: 'Karui Fortress' })).toBeInTheDocument();
  });

  it('renders wiki links in notes', () => {
    const objectives = [
      createMockObjective({
        text: 'Main objective',
        notes: 'Talk to Nessa',
      }),
    ];

    render(<StepObjectiveList {...defaultProps} objectives={objectives} wikiItems={['Nessa']} />);

    expect(screen.getByRole('button', { name: 'Nessa' })).toBeInTheDocument();
  });
});
