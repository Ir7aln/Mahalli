export default defineNuxtRouteMiddleware(async (to) => {
  const localePath = useLocalePath();
  const onboardingPath = localePath("/onboarding");
  const { status, loaded, refreshStatus } = useDatabaseBootstrap();

  if (!loaded.value) {
    await refreshStatus();
  }

  if (!status.value.has_any_database && to.path !== onboardingPath) {
    return navigateTo(onboardingPath);
  }

  if (status.value.has_any_database && status.value.has_active_database && to.path === onboardingPath) {
    return navigateTo(localePath("/"));
  }
});
