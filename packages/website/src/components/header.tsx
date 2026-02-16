export function Header() {
  return (
    <header className="border-b border-stone-800 bg-stone-950">
      <nav className="mx-auto max-w-6xl px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="font-cursive text-xl text-ember-400">POE2 Overlord</div>
          <div className="flex gap-6">
            <a href="/" className="text-bone-200 hover:text-ember-400">
              Home
            </a>
            <a href="/downloads" className="text-bone-200 hover:text-ember-400">
              Downloads
            </a>
            <a href="/docs" className="text-bone-200 hover:text-ember-400">
              Docs
            </a>
          </div>
        </div>
      </nav>
    </header>
  );
}
