import { Button } from '@/components/ui/button'
import { rootRoute } from '@/router-config'
import { createRoute, HeadContent, Link } from '@tanstack/react-router'

export const landingRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/',
  component: () => (
    <>
      <HeadContent />
      <LandingPage />
    </>
  ),
  head() {
    return {
      meta: [{ title: 'Sipdah' }],
    }
  },
})

export function LandingPage() {
  return (
    <main>
      <Button asChild>
        <Link to='/signin'>Sign in</Link>
      </Button>
      <Button asChild>
        <Link to='/signin'>Sign up</Link>
      </Button>
    </main>
  )
}
