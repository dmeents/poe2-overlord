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

  describe('Static Rendering', () => {
    it('renders window title controls correctly', () => {
      const { container } = render(<WindowTitle />);

      // Drag region
      const dragRegion = container.querySelector('[data-tauri-drag-region]');
      expect(dragRegion).toBeInTheDocument();

      // Control buttons
      expect(screen.getByRole('button', { name: 'Minimize window' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Maximize window' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Close window' })).toBeInTheDocument();
    });
  });

  it('calls minimize when minimize button is clicked', async () => {
    const user = userEvent.setup();
    render(<WindowTitle />);

    await user.click(screen.getByRole('button', { name: 'Minimize window' }));

    expect(mockMinimize).toHaveBeenCalledTimes(1);
  });

  it('calls toggleMaximize when maximize button is clicked', async () => {
    const user = userEvent.setup();
    render(<WindowTitle />);

    await user.click(screen.getByRole('button', { name: 'Maximize window' }));

    expect(mockToggleMaximize).toHaveBeenCalledTimes(1);
  });

  it('calls close when close button is clicked', async () => {
    const user = userEvent.setup();
    render(<WindowTitle />);

    await user.click(screen.getByRole('button', { name: 'Close window' }));

    expect(mockClose).toHaveBeenCalledTimes(1);
  });
});
