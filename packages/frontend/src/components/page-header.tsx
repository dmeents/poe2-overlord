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
    <div className='mb-8'>
      <div className='flex items-center justify-between mb-4'>
        <h1 className='text-3xl font-bold text-zinc-100 font-cusrive'>
          {title}
        </h1>
        <div className='flex items-center gap-2'>
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
      <p className='mt-2 text-zinc-400'>{subtitle}</p>
    </div>
  );
}
