import { Component, type ReactNode } from 'react';
import { Button } from '@/components/ui/button/button';
import type { AppError } from '@/types/error';
import { parseError } from '@/utils/error-handling';

interface Props {
  children: ReactNode;
  fallback?: (error: AppError, reset: () => void) => ReactNode;
}

interface State {
  error: AppError | null;
}

export class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = { error: null };
  }

  static getDerivedStateFromError(error: unknown): State {
    return { error: parseError(error) };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    console.error('Error boundary caught error:', error, errorInfo);
  }

  reset = () => {
    this.setState({ error: null });
  };

  render() {
    if (this.state.error) {
      if (this.props.fallback) {
        return this.props.fallback(this.state.error, this.reset);
      }

      return (
        <div className="flex items-center justify-center min-h-screen p-8">
          <div className="max-w-md w-full bg-stone-800 border border-stone-700 rounded-lg p-6 text-center">
            <div className="text-blood-400 mb-4">
              <svg
                className="mx-auto h-12 w-12"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                aria-label="Error icon">
                <title>Error</title>
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                />
              </svg>
            </div>
            <h2 className="text-xl font-semibold text-stone-100 mb-2">Something went wrong</h2>
            <p className="text-stone-400 mb-4">{this.state.error.message}</p>
            <Button type="button" onClick={this.reset} variant="primary" size="sm">
              Try again
            </Button>
          </div>
        </div>
      );
    }

    return this.props.children;
  }
}
