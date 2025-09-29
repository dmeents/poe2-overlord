interface CharacterCardSkeletonProps {
  className?: string;
}

export function CharacterCardSkeleton({
  className = '',
}: CharacterCardSkeletonProps) {
  return (
    <div
      className={`bg-zinc-900/50 p-6 rounded-lg border border-zinc-800 animate-pulse ${className}`}
    >
      <div className='flex items-center gap-3 mb-5'>
        {/* Level skeleton */}
        <div className='w-8 h-8 bg-zinc-700 rounded-full'></div>
        {/* Name skeleton */}
        <div className='h-6 bg-zinc-700 rounded w-3/4'></div>
      </div>

      {/* Details skeleton */}
      <div className='flex items-center gap-6 mb-2'>
        <div className='space-y-1'>
          <div className='h-3 bg-zinc-700 rounded w-8'></div>
          <div className='h-4 bg-zinc-700 rounded w-16'></div>
        </div>
        <div className='space-y-1'>
          <div className='h-3 bg-zinc-700 rounded w-12'></div>
          <div className='h-4 bg-zinc-700 rounded w-20'></div>
        </div>
        <div className='space-y-1'>
          <div className='h-3 bg-zinc-700 rounded w-8'></div>
          <div className='h-4 bg-zinc-700 rounded w-16'></div>
        </div>
      </div>

      {/* Footer skeleton */}
      <div className='px-5 py-4 bg-gradient-to-r from-zinc-900/50 to-zinc-800/30 border-t border-zinc-700/50'>
        <div className='space-y-3'>
          <div className='flex items-center justify-between'>
            <div className='h-3 bg-zinc-700 rounded w-16'></div>
            <div className='h-4 bg-zinc-700 rounded w-20'></div>
          </div>
          <div className='flex items-center justify-between'>
            <div className='h-3 bg-zinc-700 rounded w-12'></div>
            <div className='h-4 bg-zinc-700 rounded w-16'></div>
          </div>
        </div>
      </div>
    </div>
  );
}
