import { createRootRoute, createRoute, createRouter, Outlet } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/router-devtools'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'

import { landingRoute } from '@/pages/landing/landing-page'
import { QueryClientProvider } from '@tanstack/react-query'
import { queryClient } from '@/lib/query'
import { signInRoute } from '@/pages/auth/signin-page'
import { signUpRoute } from '@/pages/auth/signup-page'

export const rootRoute = createRootRoute({
  head: () => {
    return {
      meta: [
        {
          title: 'Sipdah',
        },
      ],
    }
  },
  component: () => (
    <>
      <QueryClientProvider client={queryClient}>
        <Outlet />
        <TanStackRouterDevtools />
        <ReactQueryDevtools initialIsOpen={false} />
      </QueryClientProvider>
    </>
  ),
})

const routeTree = rootRoute.addChildren([landingRoute, signInRoute, signUpRoute])

export const router = createRouter({
  routeTree,
  defaultPreload: 'intent',
  scrollRestoration: true,
  defaultPendingMinMs: 0,
})

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}
