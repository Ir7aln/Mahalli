<template>
  <main
    class="min-h-screen bg-[radial-gradient(circle_at_top,_rgba(14,165,233,0.18),_transparent_38%),linear-gradient(180deg,_#f8fafc_0%,_#eef6ff_100%)]"
  >
    <div class="mx-auto flex min-h-screen w-full max-w-6xl items-center px-6 py-12">
      <slot />
    </div>
    <Toaster position="top-center" />
  </main>
</template>
