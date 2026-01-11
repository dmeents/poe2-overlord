import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';

// Use vi.hoisted to ensure mocks are hoisted properly
const { mockMinimize, mockToggleMaximize, mockClose } = vi.hoisted(() => ({
  mockMinimize: vi.fn(),
  mockToggleMaximize: vi.fn(),
  mockClose: vi.fn(),
}));

vi.mock('@tauri-apps/api/window', () => ({
  getCurrentWindow: () => ({
    minimize: mockMinimize,
    toggleMaximize: mockToggleMaximize,
    close: mockClose,
  }),
}));

import { WindowTitle } from './window-title';

describe('WindowTitle', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders title text', () => {
    render(<WindowTitle />);

    expect(screen.getByText('POE Overlord')).toBeInTheDocument();
  });

  it('renders minimize button', () => {
    render(<WindowTitle />);

    expect(
      screen.getByRole('button', { name: 'minimize' })
    ).toBeInTheDocument();
  });

  it('renders maximize button', () => {
    render(<WindowTitle />);

    expect(
      screen.getByRole('button', { name: 'maximize' })
    ).toBeInTheDocument();
  });

  it('renders close button', () => {
    render(<WindowTitle />);

    expect(screen.getByRole('button', { name: 'close' })).toBeInTheDocument();
  });

  it('calls minimize when minimize button is clicked', async () => {
    const user = userEvent.setup();
    render(<WindowTitle />);

    await user.click(screen.getByRole('button', { name: 'minimize' }));

    expect(mockMinimize).toHaveBeenCalledTimes(1);
  });

  it('calls toggleMaximize when maximize button is clicked', async () => {
    const user = userEvent.setup();
    render(<WindowTitle />);

    await user.click(screen.getByRole('button', { name: 'maximize' }));

    expect(mockToggleMaximize).toHaveBeenCalledTimes(1);
  });

  it('calls close when close button is clicked', async () => {
    const user = userEvent.setup();
    render(<WindowTitle />);

    await user.click(screen.getByRole('button', { name: 'close' }));

    expect(mockClose).toHaveBeenCalledTimes(1);
  });
});
