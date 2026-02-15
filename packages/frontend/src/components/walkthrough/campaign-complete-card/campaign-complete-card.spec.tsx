import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { createMockCharacter } from '../../../test/mock-data';
import { CampaignCompleteCard } from './campaign-complete-card';

describe('CampaignCompleteCard', () => {
  const defaultProps = {
    lastUpdated: '2024-01-15T12:00:00Z',
  };

  describe('Static Rendering', () => {
    it('renders campaign complete card information correctly', () => {
      render(<CampaignCompleteCard {...defaultProps} />);

      // Title
      expect(screen.getByText('Campaign Conquered')).toBeInTheDocument();

      // Completion message
      expect(screen.getByText(/Your journey through Wraeclast is complete/)).toBeInTheDocument();
    });
  });

  it('formats completion date correctly', () => {
    render(<CampaignCompleteCard {...defaultProps} />);

    expect(screen.getByText(/January 15, 2024/)).toBeInTheDocument();
  });

  it('shows completion time when character data is provided', () => {
    const character = createMockCharacter({
      summary: {
        character_id: 'test',
        total_play_time: 0,
        total_hideout_time: 0,
        total_town_time: 0,
        total_zones_visited: 0,
        total_deaths: 0,
        play_time_act1: 3600, // 1 hour
        play_time_act2: 3600,
        play_time_act3: 3600,
        play_time_act4: 3600,
        play_time_act5: 3600,
        play_time_interlude: 3600, // Total: 6 hours
        play_time_endgame: 0,
      },
    });

    render(<CampaignCompleteCard {...defaultProps} character={character} />);

    expect(screen.getByText(/Completed in 6 hours on/)).toBeInTheDocument();
  });

  it('uses singular "hour" for 1 hour completion time', () => {
    const character = createMockCharacter({
      summary: {
        character_id: 'test',
        total_play_time: 0,
        total_hideout_time: 0,
        total_town_time: 0,
        total_zones_visited: 0,
        total_deaths: 0,
        play_time_act1: 1800, // 0.5 hours
        play_time_act2: 1800, // Total: 1 hour (rounded)
        play_time_act3: 0,
        play_time_act4: 0,
        play_time_act5: 0,
        play_time_interlude: 0,
        play_time_endgame: 0,
      },
    });

    render(<CampaignCompleteCard {...defaultProps} character={character} />);

    expect(screen.getByText(/Completed in 1 hour on/)).toBeInTheDocument();
  });

  it('renders view guide button when onViewGuide is provided', () => {
    const handleViewGuide = vi.fn();

    render(<CampaignCompleteCard {...defaultProps} onViewGuide={handleViewGuide} />);

    expect(screen.getByRole('button', { name: /guide/i })).toBeInTheDocument();
  });

  it('does not render view guide button when onViewGuide is not provided', () => {
    render(<CampaignCompleteCard {...defaultProps} />);

    expect(screen.queryByRole('button', { name: /guide/i })).not.toBeInTheDocument();
  });

  it('calls onViewGuide when button is clicked', async () => {
    const user = userEvent.setup();
    const handleViewGuide = vi.fn();

    render(<CampaignCompleteCard {...defaultProps} onViewGuide={handleViewGuide} />);

    await user.click(screen.getByRole('button', { name: /guide/i }));

    expect(handleViewGuide).toHaveBeenCalledTimes(1);
  });

  it('applies custom className', () => {
    const { container } = render(
      <CampaignCompleteCard {...defaultProps} className="custom-class" />,
    );

    const card = container.firstChild as HTMLElement;
    expect(card.className).toContain('custom-class');
  });
});
