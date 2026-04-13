<template>
  <main class="w-screen h-screen fixed inset-0 grainy-light flex">
    <SideBar />
    <div class="grid grid-rows-[48px_1fr] w-full">
      <Navigation />
      <ScrollArea class="scroll-smooth">
        <div class="min-h-[calc(100vh-48px)] p-2">
          <slot />
        </div>
      </ScrollArea>
    </div>
    <ModalsWrapper />
    <Toaster position="top-center" />
  </main>
</template>
