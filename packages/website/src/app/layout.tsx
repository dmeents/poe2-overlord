import type { Metadata } from 'next';
import { Macondo, Roboto } from 'next/font/google';
import './globals.css';

const roboto = Roboto({
  subsets: ['latin'],
  weight: ['100', '300', '400', '500', '700', '900'],
  variable: '--font-sans',
  display: 'swap',
});

const macondo = Macondo({
  weight: '400',
  subsets: ['latin'],
  variable: '--font-cursive',
  display: 'swap',
});

export const metadata: Metadata = {
  title: 'POE2 Overlord - Game Overlay for Path of Exile 2',
  description: 'Track characters, zone statistics, campaign progress, and more.',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className={`${roboto.variable} ${macondo.variable}`}>
      <body className="font-sans antialiased">{children}</body>
    </html>
  );
}
