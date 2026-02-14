export * from '../shared-modal.styles';

export function getWarningIconClasses(): string {
  return 'h-6 w-6 text-blood-400';
}

export function getModalContentClasses(): string {
  return 'mb-6';
}

export function getModalActionsClasses(): string {
  return 'flex justify-end gap-3';
}

export function getDeleteButtonClasses(): string {
  return 'text-blood-400 hover:text-blood-300 hover:border-blood-400';
}
