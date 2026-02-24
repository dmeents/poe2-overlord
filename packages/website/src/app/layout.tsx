import type { Metadata } from 'next';
import { Cinzel, Roboto } from 'next/font/google';
import './globals.css';
import { Footer } from '../components/footer/footer.component';
import { Navigation } from '../components/navigation/navigation.component';

const roboto = Roboto({
  subsets: ['latin'],
  weight: ['100', '300', '400', '500', '700', '900'],
  variable: '--font-sans',
  display: 'swap',
});

const cinzel = Cinzel({
  weight: ['400', '500', '600', '700', '800', '900'],
  subsets: ['latin'],
  variable: '--font-cursive',
  display: 'swap',
});

export const metadata: Metadata = {
  title: 'POE2 Overlord - Companion App for Path of Exile 2',
  description: 'Track characters, zone statistics, campaign progress, and more.',
  icons: {
    icon: [
      { url: '/favicon.ico', sizes: '16x16 32x32 48x48' },
      { url: '/icon-192.png', sizes: '192x192', type: 'image/png' },
      { url: '/icon-512.png', sizes: '512x512', type: 'image/png' },
    ],
    apple: [{ url: '/apple-touch-icon.png', sizes: '180x180', type: 'image/png' }],
  },
  manifest: '/manifest.json',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className={`${roboto.variable} ${cinzel.variable}`}>
      <body className="font-sans antialiased text-stone-50 bg-linear-to-br from-stone-900 via-stone-900 to-stone-950">
        <Navigation />
        <div>{children}</div>
        <Footer />
      </body>
    </html>
  );
}
