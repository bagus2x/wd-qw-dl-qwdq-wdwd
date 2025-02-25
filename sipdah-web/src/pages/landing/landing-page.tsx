import { Button } from '@/components/ui/button'
import { useCurrentUser } from '@/data/user-hooks'
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
  const user = useCurrentUser()

  return (
    <main>
      <pre>{JSON.stringify(user.isFetching, null, 2)}</pre>
      <pre>{JSON.stringify(user.data, null, 2)}</pre>
      <Button asChild>
        <Link to='/signin'>Sign in</Link>
      </Button>
      <Button asChild>
        <Link to='/signup'>Sign up</Link>
      </Button>
    </main>
  )
}
