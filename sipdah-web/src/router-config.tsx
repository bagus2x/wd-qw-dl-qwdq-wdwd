import { createRootRoute, createRoute, createRouter, Outlet } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/router-devtools'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'

import { landingRoute } from '@/pages/landing/landing-page'
import { QueryClientProvider } from '@tanstack/react-query'
import { queryClient } from '@/lib/query'

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

const signInRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/signin',
  head() {
    return {
      meta: [{ title: 'Login to sipdah!' }],
    }
  },
}).lazy(() => import('@/pages/auth/signin-page').then((d) => d.signInLazyRoute))

const signUpRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/signup',
  head() {
    return {
      meta: [{ title: 'Register to sipdah!' }],
    }
  },
}).lazy(() => import('@/pages/auth/signup-page').then((d) => d.signUpLazyRoute))

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
