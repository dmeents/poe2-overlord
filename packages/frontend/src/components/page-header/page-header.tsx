import { ArrowLeftIcon } from '@heroicons/react/24/outline';
import { useNavigate } from '@tanstack/react-router';
import { Button } from '../button';
import { pageHeaderStyles } from './page-header.styles';

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
    <div className={pageHeaderStyles.container}>
      <div className={pageHeaderStyles.content}>
        <div className={pageHeaderStyles.header}>
          <div className={pageHeaderStyles.titleSection}>
            <h1 className={pageHeaderStyles.title}>{title}</h1>
            <p className={pageHeaderStyles.subtitle}>{subtitle}</p>
          </div>

          <div className={pageHeaderStyles.actions}>
            {rightContent}
            {showBackButton && (
              <Button
                variant='outline'
                size='sm'
                onClick={handleBackClick}
                className={pageHeaderStyles.backButton}
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
