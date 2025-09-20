import { ArrowLeftIcon } from '@heroicons/react/24/outline';
import { useNavigate } from '@tanstack/react-router';
import { Button } from './button';

interface PageHeaderProps {
  title: string;
  subtitle: string;
  showBackButton?: boolean;
  backTo?: string;
  rightContent?: React.ReactNode;
}

export function PageHeader({
  title,
  subtitle,
  showBackButton = true,
  backTo = '/',
  rightContent,
}: PageHeaderProps) {
  const navigate = useNavigate();

  const handleBackClick = () => {
    navigate({ to: backTo });
  };

  return (
    <div className='w-full bg-gradient-to-r from-zinc-900 via-zinc-800/50 to-zinc-900 border-b border-zinc-800/50 mb-8'>
      <div className='max-w-7xl mx-auto px-6 py-8'>
        <div className='flex items-start justify-between'>
          <div className='flex-1 min-w-0'>
            <h1 className='text-3xl font-bold text-white font-cursive tracking-tight mb-3'>
              {title}
            </h1>
            <p className='text-zinc-300 text-lg leading-relaxed max-w-3xl'>
              {subtitle}
            </p>
          </div>
          
          <div className='ml-6 flex items-center gap-3 shrink-0'>
            {rightContent}
            {showBackButton && (
              <Button
                variant='outline'
                size='sm'
                onClick={handleBackClick}
                className='flex items-center gap-2'
              >
                <ArrowLeftIcon className='w-4 h-4' />
                Back
              </Button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
