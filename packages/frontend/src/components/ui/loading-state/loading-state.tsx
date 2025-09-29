interface LoadingStateProps {
  itemCount?: number;
  variant?: 'grid' | 'list' | 'card';
  className?: string;
}

export function LoadingState({
  itemCount = 4,
  variant = 'grid',
  className = '',
}: LoadingStateProps) {
  const renderLoadingItems = () => {
    const items = Array.from({ length: itemCount }, (_, i) => (
      <div key={i} className='animate-pulse'>
        <div className='h-4 bg-zinc-700 rounded mb-2'></div>
        <div className='h-6 bg-zinc-700 rounded'></div>
      </div>
    ));

    switch (variant) {
      case 'grid':
        return <div className='grid grid-cols-2 gap-4'>{items}</div>;
      case 'list':
        return <div className='space-y-4'>{items}</div>;
      case 'card':
        return <div className='space-y-3'>{items}</div>;
      default:
        return items;
    }
  };

  return <div className={`${className}`}>{renderLoadingItems()}</div>;
}
