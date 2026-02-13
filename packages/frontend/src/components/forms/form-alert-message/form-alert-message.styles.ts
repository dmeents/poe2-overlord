// Form Alert Message Styles
// Centralized styling utilities for the AlertMessage component
// Uses blood (danger) and verdant (success) color palette from theme

export const formAlertMessageStyles = {
  container: 'px-4 py-3',
  error: 'bg-blood-950/20 border border-blood-800 text-blood-400',
  success: 'bg-verdant-950/20 border border-verdant-800 text-verdant-400',
} as const;
