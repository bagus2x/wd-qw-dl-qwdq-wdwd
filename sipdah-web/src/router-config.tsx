import { landingRoute } from '@/pages/landing/landing-page'
import { createRootRoute, createRoute, createRouter } from '@tanstack/react-router'

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
})

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}
